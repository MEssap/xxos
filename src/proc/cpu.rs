use super::{context::Context, process::Proc};

#[allow(unused)]
pub struct RiscvCpu {
    proc: *mut Proc,       // 没有进程运行在cpu上时为null
    context: *mut Context, // 进程上下文
}
