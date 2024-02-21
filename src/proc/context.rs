#![allow(unused)]

use core::arch::{asm, global_asm};

global_asm!(include_str!("switch.s"));

#[repr(C)]
#[derive(Debug)]
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
    pub unsafe fn store_context(&self) {
        extern "C" {
            fn _store_context(context: &Context);
        }
        _store_context(self);
    }

    #[inline]
    pub unsafe fn load_context(&self) {
        extern "C" {
            fn _load_context(context: &Context);
        }
        _load_context(self);
    }

    pub fn test(&mut self, s1: usize) {
        self.s1 = s1;
    }

    pub unsafe fn scheduler() {}
}
