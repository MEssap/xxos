#![allow(unused)]

use core::arch::asm;

#[repr(C)]
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

    #[inline]
    pub unsafe fn store_to(&self) {
        asm!(
            "sd ra, 0({0})\n
             sd sp, 8({0})\n
             sd s0, 16({0})\n
             sd s1, 24({0})\n
             sd s2, 32({0})\n
             sd s3, 40({0})\n
             sd s4, 48({0})\n
             sd s5, 56({0})\n
             sd s6, 64({0})\n
             sd s7, 72({0})\n
             sd s8, 80({0})\n
             sd s9, 88({0})\n
             sd s10, 96({0})\n
             sd s11, 104({0})\n",
            in(reg) self,
        );
    }

    #[inline]
    pub unsafe fn load_context(&mut self) {
        let mut self_ptr = self as *mut Context;
        asm!(
            "ld ra, 0({0})\n
             ld sp, 8({0})\n
             ld s0, 16({0})\n
             ld s1, 24({0})\n
             ld s2, 32({0})\n
             ld s3, 40({0})\n
             ld s4, 48({0})\n
             ld s5, 56({0})\n
             ld s6, 64({0})\n
             ld s7, 72({0})\n
             ld s8, 80({0})\n
             ld s9, 88({0})\n
             ld s10, 96({0})\n
             ld s11, 104({0})\n",
            inout(reg) self_ptr,
        );
    }

    pub unsafe fn scheduler() {}
}
