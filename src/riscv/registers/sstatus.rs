use super::RegisterOperator;
use core::arch::asm;

/// regiter sstatus(Supervisor Status Register)
pub struct Sstatus {
    bits: usize,
}

// Supervisor Previous Privilege Mode
pub enum SPP {
    Machine = 0b11,
    Supervisor = 0b01,
    User = 0b00,
}

impl Sstatus {
    pub fn bits(&self) -> usize {
        self.bits
    }

    pub fn set(&mut self, bits: usize) {
        self.bits = bits;
    }
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

impl RegisterOperator for Sstatus {
    #[inline]
    fn read() -> Self {
        let bits: usize;
        unsafe { asm!("csrr {}, sstatus", out(reg) bits) }

        Self { bits }
    }

    #[inline]
    fn write(&self) {
        unsafe { asm!("csrw sstatus, {}", in(reg) self.bits) }
    }

    #[inline]
    fn _clear(&self, bits: usize) {
        unsafe { asm!("csrc sstatus, {}", in(reg) bits) }
    }

    #[inline]
    fn _set(&self, bits: usize) {
        unsafe { asm!("csrs sstatus, {}", in(reg) bits) }
    }
}
