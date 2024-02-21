#![allow(unused)]

use core::{arch::asm, mem::size_of};

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
        self.bits & !(1 << (size_of::<usize>() * 8 - 1))
    }
}

#[inline]
pub fn read() -> Scause {
    let bits: usize;
    unsafe {
        asm!("csrr {}, scause", out(reg) bits);
    }
    Scause { bits }
}
