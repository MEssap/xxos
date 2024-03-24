use crate::{
    mm::{
        def::PGSZ,
        page_frame::alloc_page,
        pagetable_frame::PageTableFrame,
        pm::def::{TRAMPOLINE, TRAPFRAME},
    },
    proc::process::Tcb,
    riscv::{
        registers::satp::Satp,
        sv39::pteflags::{PTE_FLAG_R, PTE_FLAG_V, PTE_FLAG_W, PTE_FLAG_X},
    },
};
use alloc::boxed::Box;

// User Virtual Memory
// 完成用户态的虚拟内存映射，采用随机映射的方式
/// # Unsafety
/// 不稳定的数据结构，现阶段先不使用，待文件系统完成时使用
/// 涉及到数据读取之类的
#[derive(Default)]
pub struct Uvm {
    pagetables: Box<PageTableFrame>,
}

impl Uvm {
    pub fn map_trap(&mut self, trapframe: usize) -> &mut Self {
        extern "C" {
            fn strampsec();
        }

        // map trapvec code
        self.pagetables.mappages(
            TRAMPOLINE.into(),
            (strampsec as usize).into(),
            PGSZ,
            PTE_FLAG_V | PTE_FLAG_R | PTE_FLAG_X,
        );

        self.pagetables.mappages(
            TRAPFRAME.into(),
            trapframe.into(),
            PGSZ,
            PTE_FLAG_R | PTE_FLAG_W | PTE_FLAG_X | PTE_FLAG_V,
        );

        self
    }

    /// # Safety
    /// 这个函数不稳定,之后可能需要结合src copy一起使用，现在先不使用
    pub unsafe fn mappages(&mut self, va: usize, size: usize, flags: usize) -> &mut Self {
        let mut va = va;
        let last = va + size;
        while va < last {
            let page = alloc_page();
            let pa = page.to_pma();
            self.pagetables.save_page(page);
            self.pagetables.mappages(va.into(), pa, PGSZ, flags);
            va += PGSZ;
        }
        self
    }

    pub fn as_satp(&self) -> Satp {
        let ppn = self.pagetables.root().to_ppn();
        let mut satp = Satp::new();
        satp.set_mode(crate::riscv::registers::satp::Mode::Sv39);
        satp.set_ppn(ppn.0);
        satp
    }
}
