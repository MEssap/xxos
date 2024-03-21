mod def;

use core::arch::asm;
use def::*;

pub struct Opensbi;

impl Opensbi {
    pub fn console_putchar(c: usize) {
        sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0, 0);
    }

    // 启动硬件线程(hart, Hardware Thread)
    // 在risc-v中，一个hart就是一个CPU
    pub fn sbi_hsm_hart_start(hart_id: usize) -> usize {
        sbi_call(SBI_EXT_HSM, hart_id, 0x80200000, 64, SBI_EXT_HSM_HART_START)
    }

    pub fn shutdown() -> ! {
        sbi_call(SBI_SHUTDOWN, 0, 0, 0, 0);
        panic!("It should shutdown!");
    }

    pub fn sbi_set_timer(stime_value: usize) {
        sbi_call(SBI_SET_TIMER, stime_value, 0, 0, 0);
    }
}

#[inline]
pub fn r_tp() -> usize {
    unsafe {
        let id;
        asm!("mv {0}, tp", out(reg) id);
        id
    }
}

pub fn thread_start() {
    // use crate::println;
    // println!("hello");
    let tp = r_tp();
    let i: usize = 0;
    for i in i..N_HART {
        if i != tp {
            Opensbi::sbi_hsm_hart_start(i);
        }
    }
}

#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
        "ecall",
        inlateout("x10") arg0 => ret,
        in("x11") arg1,
        in("x12") arg2,
        in("x16") arg3,
        in("x17") which,
        );
    }
    ret
}
