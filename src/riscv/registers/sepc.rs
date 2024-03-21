use core::arch::asm;

#[derive(Debug, Default, Clone)]
pub struct Sepc {
    bits: usize,
}

impl Sepc {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn bits(&self) -> usize {
        self.bits
    }

    #[inline]
    pub fn read() -> Self {
        let bits: usize;
        unsafe { asm!("csrr {}, sepc", out(reg) bits) }
        Self { bits }
    }

    #[inline]
    pub fn address(&self) -> usize {
        self.bits
    }

    #[inline]
    pub fn set_address(&mut self, addr: usize) {
        self.bits = addr;
    }

    #[inline]
    pub fn write(&self) {
        unsafe { asm!("csrw sepc, {}", in(reg) self.bits) };
    }
}