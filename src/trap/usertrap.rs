use crate::{
    mm::{
        pagetable_frame,
        pagetable_frame::PageTableFrame,
        pm::def::{kstack, KERNEL_STACK_SIZE, TRAMPOLINE},
    },
    proc::{process::Tcb, TASKMANAGER},
    riscv::{
        self,
        registers::{
            r_tp,
            satp::Satp,
            sepc,
            sstatus::{self, intr_off, intr_on},
            stvec,
        },
    },
    trap::{strampsec, userret, uservec},
};
use xxos_log::warn;

#[no_mangle]
pub extern "C" fn usertrapret() {
    fn as_satp(pagetable: &PageTableFrame) -> Satp {
        let ppn = pagetable.root().to_ppn();
        let mut satp = Satp::new();
        satp.set_mode(crate::riscv::registers::satp::Mode::Sv39);
        satp.set_ppn(ppn.0);
        satp
    }

    intr_off();

    // 设置用户中断向量表(保存虚拟地址)
    stvec::Stvec::write(
        TRAMPOLINE + (uservec as usize - strampsec as usize),
        stvec::TrapMode::Direct,
    );

    let mut task: alloc::sync::Arc<Tcb> = TASKMANAGER.lock().pop().expect("No Task in Manger");
    let pid = task.pid();
    let trapframe: &mut crate::proc::process::TrapFrame =
        task.get_mut_trapframe().expect("get trapframe err");
    trapframe.kernel_satp = riscv::registers::satp::Satp::read().bits();
    trapframe.kernel_sp = kstack(*pid) + KERNEL_STACK_SIZE;
    trapframe.kernel_trap = usertrap as usize;
    trapframe.kernel_hartid = r_tp();
    sstatus::Sstatus::set_spp(sstatus::SPP::User);
    sstatus::Sstatus::set_spie();
    sepc::Sepc::_write(trapframe.epc);
    let satp = as_satp(task.pagetable()).bits();
    let next_fn: usize = TRAMPOLINE + (userret as usize - strampsec as usize);
    unsafe {
        let fn_0: extern "C" fn(usize) -> ! = core::mem::transmute(next_fn);
        fn_0(satp) // (*next_fn)(satp)
    }
}

#[no_mangle]
pub fn usertrap() {
    // TODO: set trap handler
    warn!("run into usertrap");
    panic!("loop in usertrap")
}
