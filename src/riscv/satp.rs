#![allow(unused)]

// satp register
pub struct Satp {
    bits: usize,
}

// satp mode
pub enum Mode {
    Bare = 0,  // No translation or protection
    Sv39 = 8,  // Page-based 39-bit virtual addressing
    Sv48 = 9,  // Page-based 48-bit virtual addressing
    Sv57 = 10, // Page-based 57-bit virtual addressing
    Sv64 = 11, // Page-based 64-bit virtual addressing
}

impl Satp {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    // Current address-translation scheme
    #[inline]
    pub fn mode(&self) -> Mode {
        match self.bits >> 60 {
            0 => Mode::Bare,
            8 => Mode::Sv39,
            9 => Mode::Sv48,
            10 => Mode::Sv57,
            11 => Mode::Sv64,
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

pub mod satp {
    use super::Satp;
    use core::arch::asm;

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
    pub unsafe fn clear(bits: usize) {
        asm!("csrc satp, {}", in(reg) bits);
    }
}
