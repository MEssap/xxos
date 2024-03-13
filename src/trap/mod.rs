pub mod context;
pub mod def;
pub mod kerneltrap;

use core::arch::{asm, global_asm};

global_asm!(include_str!("kernelvec.s"));

pub enum TrapError {
    NoDevice,
}

pub fn trap_test() {
    unsafe { asm!("ebreak") };
}
