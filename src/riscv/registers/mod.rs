use core::arch::asm;
pub mod satp;
pub mod scause;
pub mod sepc;
pub mod sie;
pub mod sstatus;
pub mod stval;
pub mod stvec;

#[inline]
pub fn r_tp() -> usize {
    unsafe {
        let id;
        asm!("mv {0}, tp", out(reg) id);
        id
    }
}
