use core::arch::asm;

// Supervisor trap Value
pub struct Stval {
    bits: usize,
}

impl Stval {
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    #[inline]
    pub fn read() -> Self {
        let bits: usize;
        unsafe { asm!("csrr {}, stval", out(reg) bits) };
        Self { bits }
    }
}
