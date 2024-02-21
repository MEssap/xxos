#![allow(unused)]
use core::mem::size_of;

use super::{address::PhysicalPageNumber, def::PGSZ};
use crate::riscv::sv39::{pteflags::*, PPN_MASK, PPN_OFFSET};

#[repr(C)]
pub struct PageTableEntry {
    bits: usize,
}

#[repr(C)]
pub struct PageTable {
    entrys: [PageTableEntry; PGSZ / size_of::<PageTableEntry>()],
}

impl PageTableEntry {
    #[inline]
    pub fn ppn(&self) -> PhysicalPageNumber {
        let mut ppn = PhysicalPageNumber::new();
        ppn.0 = (self.bits & PPN_MASK) >> PPN_OFFSET;
        ppn
    }

    #[inline]
    pub fn check_flags(&self, flags: PTEFlags) -> bool {
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
