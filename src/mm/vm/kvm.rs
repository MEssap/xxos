use alloc::boxed::Box;
use xx_mutex_lock::OnceLock;
use xxos_log::info;
use crate::{
    mm::{
        pagetable::PageTableFrame,
        pm::def::HEAP_TOP,
    },
    riscv::{
        registers::satp::Satp,
        sv39::pteflags::{PTE_FLAG_R, PTE_FLAG_V, PTE_FLAG_W, PTE_FLAG_X},
    },
};

// Kernel Virtual Memory
pub struct Kvm {
    pagetables: Box<PageTableFrame>,
}

impl Default for Kvm {
    fn default() -> Self {
        Self::new()
    }
}

impl Kvm {
    pub fn new() -> Self {
        Self {
            pagetables: Box::new(PageTableFrame::new()),
        }
    }

    pub fn init(&mut self) {
        extern "C" {
            fn stext();
            fn etext();
            fn srodata();
            fn erodata();
            fn sdata();
            fn edata();
        }

        // map text segment
        self.pagetables.mappages(
            (stext as usize).into(),
            (stext as usize).into(),
            (etext as usize) - (stext as usize),
            PTE_FLAG_R | PTE_FLAG_W | PTE_FLAG_X | PTE_FLAG_V,
        );

        // map data segment
        self.pagetables.mappages(
            (srodata as usize).into(),
            (srodata as usize).into(),
            (erodata as usize) - (srodata as usize),
            PTE_FLAG_R | PTE_FLAG_W | PTE_FLAG_V,
        );

        self.pagetables.mappages(
            (sdata as usize).into(),
            (sdata as usize).into(),
            (edata as usize) - (sdata as usize),
            PTE_FLAG_R | PTE_FLAG_W | PTE_FLAG_V,
        );

        // map Physical Memory
        self.pagetables.mappages(
            (edata as usize).into(),
            (edata as usize).into(),
            HEAP_TOP - (edata as usize),
            PTE_FLAG_R | PTE_FLAG_W | PTE_FLAG_V,
        );
        //map kernel stack

        let satp = self.as_satp();
        satp.write();
    }

    pub fn as_satp(&self) -> Satp {
        let ppn = self.pagetables.root().to_ppn();
        let mut satp = Satp::new();
        satp.set_mode(crate::riscv::registers::satp::Mode::Sv39);
        satp.set_ppn(ppn);
        satp
    }
}

pub struct LockedKvm(OnceLock<Kvm>);

impl Default for LockedKvm {
    fn default() -> Self {
        Self::new()
    }
}

impl LockedKvm {
    pub const fn new() -> Self {
        Self(OnceLock::new())
    }

    pub fn get_or_init(&self) {
        self.0.get_or_init(kvmmake);
    }
}

pub fn kvmmake() -> Kvm {
    info!("============ kvmmake start ============");
    let mut kvm = Kvm::new();
    kvm.init();
    info!("============ kvmmake end ============");
    kvm
}
