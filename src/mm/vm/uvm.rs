use alloc::boxed::Box;
use xxos_log::info;

use crate::{
    mm::pagetable::PageTableFrame,
    riscv::registers::satp::{self, Satp},
};

// User Virtual Memory
pub struct Uvm {
    pagetables: Box<PageTableFrame>,
}

impl Default for Uvm {
    fn default() -> Self {
        Self::new()
    }
}

impl Uvm {
    pub fn new() -> Self {
        Self {
            pagetables: Box::new(PageTableFrame::new()),
        }
    }

    pub fn init(&self) {
        let satp = Satp::new();
        satp::write(satp.bits());
    }

    pub fn as_satp(&self) -> Satp {
        let ppn = self.pagetables.root().ppn();
        let mut satp = Satp::new();
        satp.set_mode(crate::riscv::registers::satp::Mode::Sv39);
        satp.set_ppn(ppn);
        satp
    }
}

pub fn uvmmake() -> Uvm {
    info!("============ uvmmake start ============");
    let uvm = Uvm::new();
    uvm.init();
    info!("============ uvmmake end ============");
    uvm
}
