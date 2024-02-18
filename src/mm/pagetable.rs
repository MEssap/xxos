#![allow(unused)]
use crate::riscv::{
    pteflags::*,
    sv39::{PTE_PPN, PTE_PPN_OFFSET},
};

use super::def::PGSZ;

type PhysicalMemoryAddress = usize;
type VirtualMemoryAddress = usize;
type PhysicalPageNumber = usize;

struct PageTableEntrys {
    bits: usize,
}

pub struct PageTable {
    entrys: [PageTableEntrys; PGSZ / 8],
}

impl PageTableEntrys {
    #[inline]
    pub fn get_ppn(&self) -> PhysicalPageNumber {
        (self.bits & PTE_PPN) >> PTE_PPN_OFFSET
    }

    #[inline]
    pub fn check_flags(&self, flags: PteFlags) -> bool {
        self.bits & (flags as usize) != 0
    }

    #[inline]
    pub fn is_v(&self) -> bool {
        self.bits & PTE_FLAG_V != 0
    }

    #[inline]
    pub fn is_r(&self) -> bool {
        self.bits & PTE_FLAG_R != 0
    }

    #[inline]
    pub fn is_w(&self) -> bool {
        self.bits & PTE_FLAG_W != 0
    }

    #[inline]
    pub fn is_x(&self) -> bool {
        self.bits & PTE_FLAG_X != 0
    }

    #[inline]
    pub fn is_u(&self) -> bool {
        self.bits & PTE_FLAG_U != 0
    }

    #[inline]
    pub fn is_g(&self) -> bool {
        self.bits & PTE_FLAG_G != 0
    }

    #[inline]
    pub fn is_a(&self) -> bool {
        self.bits & PTE_FLAG_A != 0
    }

    #[inline]
    pub fn is_d(&self) -> bool {
        self.bits & PTE_FLAG_D != 0
    }
}
