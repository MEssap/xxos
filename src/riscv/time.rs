use core::arch::asm;

pub fn read_time() -> usize {
    let mut bits: usize;
    unsafe { asm!("rdtime {}",out(reg) bits) };
    bits
}

pub fn read_cycle() -> usize {
    let mut bits: usize;
    unsafe { asm!("rdcycle {}",out(reg) bits) };
    bits
}
