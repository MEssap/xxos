#![allow(unused)]
use core::arch::asm;

use crate::mm::pagetable::PhysicalPageNumber;

// SATP register have 3 fields:
// 1. PPN filed (0-43)
// 2. ASID filed(44-59)
// 3. MODE filed(60-63)
pub const SATP_PPN_WIDTH: u8 = 44;
pub const SATP_PPN_MASK: usize = (1 << SATP_PPN_WIDTH) - 1;
pub const SATP_ASID_SHIFT: u8 = 44;
pub const SATP_ASID_WIDTH: u8 = 16;
pub const SATP_ASID_MASK: usize = ((1 << (SATP_ASID_SHIFT + SATP_ASID_WIDTH)) - 1) ^ SATP_PPN_MASK;
pub const SATP_MODE_SHIFT: u8 = 60;
pub const SATP_MODE_WIDTH: u8 = 4;
pub const SATP_MODE_MASK: usize = !0 ^ (SATP_ASID_MASK | SATP_PPN_MASK);

// register satp(Supervisor Address Translation and Protection)
pub struct Satp {
    pub bits: usize,
}

// satp mode
pub enum Mode {
    Bare = 0b0000, // No translation or protection
    Sv39 = 0b1000, // Page-based 39-bit virtual addressing
    Sv48 = 0b1001, // Page-based 48-bit virtual addressing
}

impl Default for Satp {
    fn default() -> Self {
        Self::new()
    }
}

impl Satp {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn set(&mut self, bits: usize) {
        self.bits = bits;
    }

    // Current address-translation scheme
    #[inline]
    pub fn mode(&self) -> Mode {
        match (self.bits & SATP_MODE_MASK) >> SATP_MODE_SHIFT {
            0 => Mode::Bare,
            8 => Mode::Sv39,
            9 => Mode::Sv48,
            _ => unreachable!(),
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.bits |= ((mode as usize) << SATP_MODE_SHIFT);
    }

    // Address Space IDentifier
    #[inline]
    pub fn asid(&self) -> usize {
        (self.bits & SATP_ASID_MASK) >> SATP_ASID_SHIFT // bits 44-59
    }

    pub fn set_asid(&mut self, asid: usize) {
        self.bits |= (asid << SATP_ASID_SHIFT);
    }

    // Physical Page Number
    #[inline]
    pub fn ppn(&self) -> PhysicalPageNumber {
        PhysicalPageNumber::from(self.bits & SATP_PPN_MASK) // bits 0-43
    }

    pub fn set_ppn(&mut self, ppn: PhysicalPageNumber) {
        self.bits |= ppn.0;
    }
}

#[inline]
pub fn read() -> Satp {
    let mut bits = 0;
    unsafe { asm!("csrr {}, satp", out(reg) bits) }
    Satp { bits }
}

#[inline]
pub fn write(bits: usize) {
    unsafe { asm!("csrw satp, {}", in(reg) bits) }
}

#[inline]
pub fn clear(bits: usize) {
    unsafe { asm!("csrc satp, {}", in(reg) bits) }
}
