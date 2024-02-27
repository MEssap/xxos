use alloc::boxed::Box;
use xxos_log::{error, info, warn};

use crate::{
    mm::{
        def::PGSZ,
        pagetable::{PageTable, PageTableFrame, PhysicalMemoryAddress, VirtualMemoryAddress},
        pm::def::{KERNBASE, PHYSTOP},
    },
    riscv::{
        registers::satp::{write, Satp},
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
            fn ekernel();
            fn skernel();
        }

        // map text segment
        self.pagetables.mappages(
            VirtualMemoryAddress::from(KERNBASE),
            PhysicalMemoryAddress::from(KERNBASE),
            (etext as usize) - KERNBASE,
            PTE_FLAG_R | PTE_FLAG_W | PTE_FLAG_X | PTE_FLAG_V,
        );

        for i in 0..10 {
            match self
                .pagetables
                .walk(VirtualMemoryAddress::from(KERNBASE + i * PGSZ), false)
            {
                Ok(pte) => {
                    error!("{:#x?}", pte);
                }
                Err(err) => error!("{:#x?}", err),
            }
        }

        //// map Physical Memory
        //self.pagetables.mappages(
        //    VirtualMemoryAddress::from(etext as usize),
        //    PhysicalMemoryAddress::from(etext as usize),
        //    PHYSTOP - (etext as usize),
        //    PTE_FLAG_V,
        //);

        let satp = self.as_satp();
        //write(satp.bits);
    }

    pub fn as_satp(&self) -> Satp {
        let ppn = self.pagetables.root().ppn();
        let mut satp = Satp::new();
        satp.set_mode(crate::riscv::registers::satp::Mode::Sv39);
        satp.set_ppn(ppn);
        satp
    }
}

pub fn kvmmake() -> Kvm {
    info!("============ kvmmake start ============");
    let mut kvm = Kvm::new();
    kvm.init();
    info!("============ kvmmake end ============");
    kvm
}
