use crate::{
    riscv::registers::{
        scause::{Exception, Interrupt, Scause, Trap},
        sepc::Sepc,
        sstatus::{intr_off, Sstatus},
        stval::Stval,
        stvec::{Stvec, TrapMode},
    },
    trap::{clock::clock_set_next_event, def::CLOCK_COUNTS, kernelvec, strampsec},
};
use xxos_log::{error, warn};

/// 由于基于OpenSBI，内核在运行之初MIDELEG寄存器的值为0x0000000000001666，
/// 即将软件中断和时钟中断委托给了S模式
/// MEDELEG寄存器的值为0x0000000000f0b509，
/// 即将未对齐指令、断点、来自用户模式的系统调用处理、指令缺页、加载缺页、存储/AMO缺页异常委托给了S模式

pub fn kernel_trap_init() {
    // 设置中断向量表(保存物理地址)
    Stvec::write(kernelvec as usize, TrapMode::Direct);
    let mut sstatus = Sstatus::read();
    sstatus.set_sie();
    sstatus.write();
}

#[inline]
#[no_mangle]
pub fn kernel_trap_handler() {
    intr_off();
    let scause = Scause::read();
    let stval = Stval::read();
    let stvec = Stvec::read();

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
        Trap::Exception(Exception::Breakpoint) => {
            let mut sepc = Sepc::read();
            let scause = Scause::read();
            let stval = Stval::read();

            warn!(
                "breakpoint sepc: {:#x?}, sscause: {:#x?}, stval = {:#x?}",
                sepc.bits(),
                scause.bits(),
                stval.bits()
            );

            sepc.set_bits(sepc.bits() + 2);
            sepc.write();
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
}
