use core::arch::asm;

// Supervisor Trap-Vector Base Address
pub struct Stvec {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrapMode {
    Direct = 0,
    Vectored = 1,
}

#[inline]
pub fn write(addr: usize, mode: TrapMode) {
    unsafe { asm!("csrw stvec, {}", in(reg) addr + mode as usize) };
}
