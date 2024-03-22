pub mod clock;
pub mod def;
//pub mod ecall;
pub mod kerneltrap;
pub mod trap_frame;

use core::arch::{asm, global_asm};

use xxos_log::{info, warn};

global_asm!(include_str!("kernelvec.s"));
//global_asm!(include_str!("uservec.s"));

extern "C" {
    pub fn kernelvec();
    //pub fn uservec();
}

pub enum TrapError {
    NoDevice,
}

pub fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}

pub fn trap_test() {
    info!("============ kvmmake start ============");
    unsafe { asm!("ebreak") };
    syscall(0xdeadbeef, [0xdeadbeef, 0xdeadbeef, 0xdeadbeef]);
    info!("============ kvmmake end ============");
}
