use super::kernelvec;
use super::trap_frame::TrapFrame;
use crate::{
    riscv::registers::{
        scause::{Exception, Interrupt, Scause, Trap},
        sstatus::{intr_off, Sstatus},
        stval::Stval,
        stvec::{Stvec, TrapMode},
    },
    trap::{clock::clock_set_next_event, def::CLOCK_COUNTS},
};
use xxos_log::{error, warn};

/// 由于基于OpenSBI，内核在运行之初MIDELEG寄存器的值为0x0000000000001666，
/// 即将软件中断和时钟中断委托给了S模式
/// MEDELEG寄存器的值为0x0000000000f0b509，
/// 即将未对齐指令、断点、来自用户模式的系统调用处理、指令缺页、加载缺页、存储/AMO缺页异常委托给了S模式

pub fn kernel_trap_init() {
    Stvec::write(kernelvec as usize, TrapMode::Direct);
    let mut sstatus = Sstatus::read();
    sstatus.set_sie();
    sstatus.write();
}

#[inline]
#[no_mangle]
pub fn kernel_trap_handler(trapframe: &mut TrapFrame) {
    intr_off();
    let scause = Scause::read();
    let stval = Stval::read();
    let stvec = Stvec::read();
    //let context = context.clone();

    match scause.cause() {
        /* 中断处理 */
        Trap::Interrupt(Interrupt::UserSoft) => {}
        Trap::Interrupt(Interrupt::SupervisorSoft) => {}
        Trap::Interrupt(Interrupt::UserTimer) => {
            warn!("UserTimer");
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            clock_set_next_event();
            if CLOCK_COUNTS.add_counts() == 100 {
                CLOCK_COUNTS.clear_counts();
                warn!("100 counts");
            }
        }
        /* 异常处理 */
        Trap::Exception(Exception::UserEnvCall) => {
            warn!(
                "syscall id {:#x?},args: [{:#x?}, {:#x?}, {:#x?}]",
                trapframe.a7(),
                trapframe.a0(),
                trapframe.a1(),
                trapframe.a2()
            );

            trapframe.set_sepc(trapframe.sepc() + 4);
            trapframe.set_a0(0);
            panic!("Now init process successfuly ecall")
        }
        Trap::Exception(Exception::Breakpoint) => {
            warn!("{:#x?}", trapframe);
            let stval = Stval::read();

            warn!(
                "breakpoint sepc: {:#x?}, sscause: {:#x?}, stval = {:#x?}",
                trapframe.sepc(),
                trapframe.scause().cause(),
                stval.bits()
            );

            trapframe.set_sepc(trapframe.sepc() + 2);
        }
        Trap::Exception(Exception::InstructionPageFault) => {
            warn!("{:#x?}", Exception::InstructionPageFault);
        }
        Trap::Exception(e) => {
            error!(
                "{:#x?} never have handler \n stvec[{:#x?}] \n  scause {:#x?}",
                e, stvec, scause
            );
            panic!("Err");
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval.bits()
            );
        }
    }

    //trap_from_kernel();
}

pub fn trap_from_kernel() {
    panic!("trap from kernel");
}
