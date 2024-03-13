use super::context::TrapContext;
use crate::{
    error,
    riscv::registers::{
        satp::Satp,
        scause::{Exception, Interrupt, Scause, Trap},
        sstatus::Sstatus,
        stval,
        stvec::{self, TrapMode},
    },
};
use xxos_log::{error, info, warn};

// 由于基于OpenSBI，内核在运行之初MIDELEG寄存器的值为0x0000000000001666，
// 即将软件中断和时钟中断委托给了S模式
// MEDELEG寄存器的值为0x0000000000f0b509，
// 即将未对齐指令、断点、来自用户模式的系统调用处理、指令缺页、加载缺页、存储/AMO缺页异常委托给了S模式

extern "C" {
    fn kernelvec();
}

pub fn kernel_trap_init() {
    stvec::write(kernelvec as usize, TrapMode::Direct);
}

#[inline]
#[no_mangle]
pub fn kerneltrap() {
    let scause = Scause::read();
    let stval = stval::read();

    match scause.cause() {
        /* 中断处理 */
        Trap::Interrupt(Interrupt::UserSoft) => {}
        Trap::Interrupt(Interrupt::SupervisorSoft) => {}
        Trap::Interrupt(Interrupt::UserTimer) => {}
        Trap::Interrupt(Interrupt::SupervisorTimer) => {}
        /* 异常处理 */
        //Trap::Exception(Exception::UserEnvCall) => {
        //    context.sepc += 4;
        //    context.x[10] =
        //        syscall(context.x[17], [context.x[10], context.x[11], context.x[12]]) as usize;
        //}
        Trap::Exception(Exception::Breakpoint) => {
            let sstatus = Sstatus::read();
            warn!("breakpoint, mode: {:#x?}", sstatus.spp());
        }
        Trap::Exception(_) => {
            error!("exception");
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }

    trap_from_kernel();
}

pub fn trap_from_kernel() {
    panic!("trap from kernel");
}
