use super::RegisterOperator;
use core::arch::asm;

// Supervisor trap Cause
// register scause
pub struct Scause {
    pub bits: usize,
}

impl Scause {
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    // Returns the code field
    #[inline]
    pub fn code(&self) -> usize {
        self.bits & !(1 << (usize::BITS - 1))
    }
}

impl RegisterOperator for Scause {
    #[inline]
    fn read() -> Self {
        let bits: usize;
        unsafe { asm!("csrr {}, scause", out(reg) bits) }

        Self { bits }
    }

    #[inline]
    fn write(&self) {
        unsafe { asm!("csrw scause, {}", in(reg) self.bits) }
    }

    #[inline]
    fn _clear(&self, bits: usize) {
        unsafe { asm!("csrc scause, {}", in(reg) bits) }
    }

    #[inline]
    fn _set(&self, bits: usize) {
        unsafe { asm!("csrs scause, {}", in(reg) bits) }
    }
}
