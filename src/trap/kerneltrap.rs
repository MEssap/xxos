use super::context::TrapContext;
use crate::{
    error,
    riscv::{
        registers::{
            satp::Satp,
            scause::{Exception, Interrupt, Scause, Trap},
            sepc::Sepc,
            sstatus::Sstatus,
            stval::Stval,
            stvec::{Stvec, TrapMode},
        },
        time,
    },
    trap::{clock::clock_set_next_event, def::CLOCK_COUNTS},
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
    Stvec::write(kernelvec as usize, TrapMode::Direct);
    let mut sstatus = Sstatus::read();
    sstatus.set_sie();
    sstatus.write();
}

#[inline]
#[no_mangle]
pub fn kernel_trap_handler() {
    let scause = Scause::read();
    let stval = Stval::read();
    //let context = context.clone();

    match scause.cause() {
        /* 中断处理 */
        Trap::Interrupt(Interrupt::UserSoft) => {}
        Trap::Interrupt(Interrupt::SupervisorSoft) => {}
        Trap::Interrupt(Interrupt::UserTimer) => {
            warn!("UserTimer");
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            warn!("SupervisorTimer");
            let time = time::read_time();
            let cycle = time::read_cycle();
            //warn!("time: {:#x?}", time);
            //warn!("cycle: {:#x?}", cycle);

            let mut sepc = Sepc::read();
            sepc.set_address(sepc.bits() + 2);
            sepc.write();

            if CLOCK_COUNTS.add_counts() == 5 {
                CLOCK_COUNTS.clear_counts();
                clock_set_next_event();
                //warn!("5 counts");
            }

            //clock_set_next_time();
        }
        /* 异常处理 */
        //Trap::Exception(Exception::UserEnvCall) => {
        //    context.sepc += 4;
        //    context.x[10] =
        //        syscall(context.x[17], [context.x[10], context.x[11], context.x[12]]) as usize;
        //}
        Trap::Exception(Exception::Breakpoint) => {
            let mut sepc = Sepc::read();
            let stval = Stval::read();

            warn!(
                "breakpoint sepc: {:#x?}, sscause: {:#x?}, stval = {:#x?}",
                sepc.bits(),
                scause.cause(),
                stval.bits()
            );

            sepc.set_address(sepc.bits() + 2);
            sepc.write();
        }
        Trap::Exception(Exception::InstructionPageFault) => {
            warn!("{:#x?}", Exception::InstructionPageFault);
        }
        Trap::Exception(e) => {
            //warn!("{:#x?}", e);
            error!("exception");
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
