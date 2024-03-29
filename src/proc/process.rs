use super::TASKMANAGER;
use crate::mm::page_frame::{alloc_page, PageFrame};
use crate::mm::pagetable_frame::PageTableFrame;
use crate::mm::pm::def::{kstack, KERNEL_STACK_SIZE, TRAMPOLINE, TRAPFRAME};
use crate::mm::vm::uvm::Uvm;
use crate::riscv::sv39::pteflags::{PTE_FLAG_R, PTE_FLAG_U, PTE_FLAG_V, PTE_FLAG_W, PTE_FLAG_X};
use crate::{cpu::Context, mm::def::PGSZ};
use alloc::string::{String, ToString};
use alloc::{
    sync::{Arc, Weak},
    vec::Vec,
};
use core::{default, ptr};
use macros::Getter;

pub static INITCODE: [u8; 52] = [
    0x17, 0x05, 0x00, 0x00, 0x13, 0x05, 0x45, 0x02, 0x97, 0x05, 0x00, 0x00, 0x93, 0x85, 0x35, 0x02,
    0x93, 0x08, 0x70, 0x00, 0x73, 0x00, 0x00, 0x00, 0x93, 0x08, 0x20, 0x00, 0x73, 0x00, 0x00, 0x00,
    0xef, 0xf0, 0x9f, 0xff, 0x2f, 0x69, 0x6e, 0x69, 0x74, 0x00, 0x00, 0x24, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
];

#[derive(Clone, Copy, Default, Debug)]
#[repr(C)]
pub struct TrapFrame {
    /*   0 */ pub kernel_satp: usize, // kernel page table
    /*   8 */ pub kernel_sp: usize, // top of process's kernel stack
    /*  16 */ pub kernel_trap: usize, // usertrap()
    /*  24 */ pub epc: usize, // saved user program counter
    /*  32 */ pub kernel_hartid: usize, // saved kernel tp
    /*  40 */ pub ra: usize,
    /*  48 */ pub sp: usize,
    /*  56 */ pub gp: usize,
    /*  64 */ pub tp: usize,
    /*  72 */ pub t0: usize,
    /*  80 */ pub t1: usize,
    /*  88 */ pub t2: usize,
    /*  96 */ pub s0: usize,
    /* 104 */ pub s1: usize,
    /* 112 */ pub a0: usize,
    /* 120 */ pub a1: usize,
    /* 128 */ pub a2: usize,
    /* 136 */ pub a3: usize,
    /* 144 */ pub a4: usize,
    /* 152 */ pub a5: usize,
    /* 160 */ pub a6: usize,
    /* 168 */ pub a7: usize,
    /* 176 */ pub s2: usize,
    /* 184 */ pub s3: usize,
    /* 192 */ pub s4: usize,
    /* 200 */ pub s5: usize,
    /* 208 */ pub s6: usize,
    /* 216 */ pub s7: usize,
    /* 224 */ pub s8: usize,
    /* 232 */ pub s9: usize,
    /* 240 */ pub s10: usize,
    /* 248 */ pub s11: usize,
    /* 256 */ pub t3: usize,
    /* 264 */ pub t4: usize,
    /* 272 */ pub t5: usize,
    /* 280 */ pub t6: usize,
}
#[derive(Debug)]
pub enum State {
    Running,
    Ready,
    Sleep,
    Zombie,
}

impl default::Default for State {
    fn default() -> Self {
        Self::Ready
    }
}
#[derive(Default, Getter)]
pub struct Tcb {
    name: String,
    state: State,
    pid: usize,
    killed: bool,
    exit_code: usize,
    parent: Option<Weak<Tcb>>,
    context: Context,
    kstack: usize,
    pagetable: PageTableFrame,
    trapframe: Option<&'static mut TrapFrame>,
    children: Vec<Arc<Tcb>>,
    frames: Vec<PageFrame>,
    vm: Uvm,
}

impl Tcb {
    // allocate memory to store data
    /// # Safety
    /// ask for 4096 size page
    pub unsafe fn alloc<T: Sized>(&mut self) -> *mut T {
        if core::mem::size_of::<T>() > PGSZ {
            panic!("Error the struct size more than a page")
        }

        let frame = alloc_page();
        let ret = frame.to_usize();

        self.frames.push(frame);
        ret as *mut T
    }

    pub fn get_mut_trapframe(&self) -> Option<&mut TrapFrame> {
        if let Some(ref ptr) = self.trapframe {
            let ptr = (*ptr) as *const _ as usize;
            unsafe { Some((ptr as *mut TrapFrame).as_mut().unwrap()) }
        } else {
            None
        }
    }
}

// 创建一个初始进程
pub fn zero_task() -> Tcb {
    fn init_zero_task_pagetable(trapframe: usize) -> PageTableFrame {
        extern "C" {
            fn strampsec();
        }

        let mut pagetable = PageTableFrame::new();
        let page = alloc_page();
        let pa = page.to_pma();

        pagetable.save_page(page);
        // map pagetable frame
        pagetable.mappages(
            0.into(),
            pa,
            PGSZ,
            PTE_FLAG_U | PTE_FLAG_V | PTE_FLAG_X | PTE_FLAG_R | PTE_FLAG_W,
        );
        // map trapvec code
        pagetable.mappages(
            TRAMPOLINE.into(),
            (strampsec as usize).into(),
            PGSZ,
            PTE_FLAG_V | PTE_FLAG_X | PTE_FLAG_R,
        );
        // map trapframe
        pagetable.mappages(
            TRAPFRAME.into(),
            trapframe.into(),
            PGSZ,
            PTE_FLAG_V | PTE_FLAG_X | PTE_FLAG_R | PTE_FLAG_W,
        );
        unsafe { ptr::copy_nonoverlapping(INITCODE.as_ptr(), pa.get_mut(), INITCODE.len()) }
        pagetable
    }

    let mut task = Tcb::default();
    let trapframe = unsafe {
        let trapframe = task.alloc::<TrapFrame>();
        (*trapframe).epc = 0;
        (*trapframe).sp = PGSZ;
        trapframe
    };

    task.name = "initcode".to_string();
    task.pid = 0;
    task.context.sp = kstack(0) + KERNEL_STACK_SIZE;
    task.context.ra = 0; //TODO: add userret
    task.trapframe = unsafe { trapframe.as_mut() };
    task.kstack = kstack(0);
    task.state = State::Ready;
    task.killed = false;
    task.pagetable = init_zero_task_pagetable(trapframe as usize);
    task
}

pub fn test_initcode() {
    let task = zero_task();
    TASKMANAGER.lock().push(Arc::new(task));
}
