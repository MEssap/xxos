#![allow(unused)]
use core::arch::asm;

/// regiter sstatus(Supervisor Status Register)
pub struct Sstatus {
    pub bits: usize,
}

// Supervisor Previous Privilege Mode
pub enum SPP {
    Machine = 0b11,
    Supervisor = 0b01,
    User = 0b00,
}

impl Sstatus {
    // Supervisor Interrupt Enable
    #[inline]
    pub fn sie(&self) -> bool {
        self.bits & (1 << 1) != 0
    }

    // Supervisor Previous Privilege Mode
    #[inline]
    pub fn spp(&self) -> SPP {
        if self.bits & (1 << 8) != 0 {
            SPP::Supervisor
        } else {
            SPP::User
        }
    }

    // Permit Supervisor User Memory access
    #[inline]
    pub fn sum(&self) -> bool {
        self.bits & (1 << 18) != 0
    }

    // Make eXecutable Readable
    #[inline]
    pub fn mxr(&self) -> bool {
        self.bits & (1 << 19) != 0
    }
}

#[inline]
pub fn read() -> Sstatus {
    let mut bits = 0;
    unsafe { asm!("csrr {}, sstatus", out(reg) bits) }
    Sstatus { bits }
}

#[inline]
pub fn write(bits: usize) {
    unsafe { asm!("csrw sstatus, {}", in(reg) bits) }
}

#[inline]
pub fn clear(bits: usize) {
    unsafe { asm!("csrc sstatus, {}", in(reg) bits) }
}
