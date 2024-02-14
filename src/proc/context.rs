#![allow(unused)]

use core::arch::asm;

use super::cpu::RiscvCpu;

pub struct Context {
    ra: usize,
    sp: usize,

    // callee-saved
    s0: usize,
    s1: usize,
    s2: usize,
    s3: usize,
    s4: usize,
    s5: usize,
    s6: usize,
    s7: usize,
    s8: usize,
    s9: usize,
    s10: usize,
    s11: usize,
}

impl Context {
    pub fn new() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s0: 0,
            s1: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,
            s8: 0,
            s9: 0,
            s10: 0,
            s11: 0,
        }
    }

    pub unsafe fn store_to(&self) {
        asm!(
            "sd ra, 0({})\n
             sd sp, 8({})\n
             sd s0, 16({})\n
             sd s1, 24({})\n
             sd s2, 32({})\n
             sd s3, 40({})\n
             sd s4, 48({})\n
             sd s5, 56({})\n
             sd s6, 64({})\n
             sd s7, 72({})\n
             sd s8, 80({})\n
             sd s9, 88({})\n
             sd s10, 96({})\n
             sd s11, 104({})\n",
            in(reg) self.ra,
            in(reg) self.sp,
            in(reg) self.s0,
            in(reg) self.s1,
            in(reg) self.s2,
            in(reg) self.s3,
            in(reg) self.s4,
            in(reg) self.s5,
            in(reg) self.s6,
            in(reg) self.s7,
            in(reg) self.s8,
            in(reg) self.s9,
            in(reg) self.s10,
            in(reg) self.s11,
        );
    }

    pub unsafe fn load_context(&mut self) {
        asm!(
            "ld ra, 0({})\n
             ld sp, 8({})\n
             ld s0, 16({})\n
             ld s1, 24({})\n
             ld s2, 32({})\n
             ld s3, 40({})\n
             ld s4, 48({})\n
             ld s5, 56({})\n
             ld s6, 64({})\n
             ld s7, 72({})\n
             ld s8, 80({})\n
             ld s9, 88({})\n
             ld s10, 96({})\n
             ld s11, 96({})\n",
            out(reg) self.ra,
            out(reg) self.sp,
            out(reg) self.s0,
            out(reg) self.s1,
            out(reg) self.s2,
            out(reg) self.s3,
            out(reg) self.s4,
            out(reg) self.s5,
            out(reg) self.s6,
            out(reg) self.s7,
            out(reg) self.s8,
            out(reg) self.s9,
            out(reg) self.s10,
            out(reg) self.s11,
        );
    }

    pub unsafe fn scheduler() {}
}
