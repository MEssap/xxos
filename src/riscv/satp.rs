#![allow(unused)]
use core::arch::asm;

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

impl Satp {
    // Current address-translation scheme
    #[inline]
    pub fn mode(&self) -> Mode {
        match self.bits >> 60 {
            0 => Mode::Bare,
            8 => Mode::Sv39,
            9 => Mode::Sv48,
            _ => unreachable!(),
        }
    }

    // Address Space IDentifier
    #[inline]
    pub fn asid(&self) -> usize {
        self.bits >> 44 & 0xFFFF // bits 44-59
    }

    // Physical Page Number
    #[inline]
    pub fn ppn(&self) -> usize {
        self.bits & 0xFFF_FFFF_FFFF // bits 0-43
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
