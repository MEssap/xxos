#![allow(unused)]

use core::ptr::null_mut;

use crate::proc::{context::Context, process::Proc};

pub struct RiscvCpu {
    proc: *mut Proc,  // 没有进程运行在cpu上时为null
    context: Context, // 进程上下文
}

impl RiscvCpu {
    pub const fn new() -> Self {
        Self {
            proc: null_mut(),
            context: Context::new(),
        }
    }
}
