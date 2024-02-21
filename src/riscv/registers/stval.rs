// Supervisor trap Value
pub struct Stval {}

use core::arch::asm;

#[inline]
pub fn read() -> usize {
    let bits: usize;
    unsafe { asm!("csrr {}, stval", out(reg) bits) }
    bits
}
