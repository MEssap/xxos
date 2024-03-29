use core::arch::asm;

// Trap Cause
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trap {
    Interrupt(Interrupt),
    Exception(Exception),
}

// 在RISC-V中，异常处理和中断处理都属于异常
// 异常分为异常和中断:
// 中断(interrupt)分为4类:
//      软件中断(software interrupt): 由软件触发，常用于处理器之间进行通信，或称为处理器间中断(Inter-Processor Interrupt, IPI)
//      定时器中断(timer interrupt): 来自定时器的中断，常用于处理器的时钟中断
//      外部中断(external interrupt): 来自处理器外部设备(如串口设备等)的中断。
//      调试中断(debug interrupt): 用于硬件调试功能
// 异常(exception)分为: 程序在执行中发生异常情况时触发
//     如指令访问异常(Instruction access fault)、数据访问异常(data access
//     fault)和缺页(page fault)等
// 系统调用(system call): 软件触发，允许软件主动通过请求更高权限的行为触发

// Interrupt
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interrupt {
    UserSoft,
    SupervisorSoft,
    UserTimer,
    SupervisorTimer,
    UserExternal,
    SupervisorExternal,
    Unknown,
}

impl Interrupt {
    #[inline]
    pub fn from(interrupt_id: usize) -> Self {
        match interrupt_id {
            0 => Interrupt::UserSoft,
            1 => Interrupt::SupervisorSoft,
            4 => Interrupt::UserTimer,
            5 => Interrupt::SupervisorTimer,
            8 => Interrupt::UserExternal,
            9 => Interrupt::SupervisorExternal,
            _ => Interrupt::Unknown,
        }
    }
}

// Exception
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Exception {
    InstructionMisaligned,
    InstructionFault,
    IllegalInstruction,
    Breakpoint,
    LoadFault,
    StoreMisaligned,
    StoreFault,
    UserEnvCall,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
    Unknown,
}

impl Exception {
    #[inline]
    pub fn from(nr: usize) -> Self {
        match nr {
            0 => Exception::InstructionMisaligned,
            1 => Exception::InstructionFault,
            2 => Exception::IllegalInstruction,
            3 => Exception::Breakpoint,
            5 => Exception::LoadFault,
            6 => Exception::StoreMisaligned,
            7 => Exception::StoreFault,
            8 => Exception::UserEnvCall,
            12 => Exception::InstructionPageFault,
            13 => Exception::LoadPageFault,
            15 => Exception::StorePageFault,
            _ => Exception::Unknown,
        }
    }
}

// Supervisor trap Cause
// register scause
#[derive(Debug, Default, Clone)]
pub struct Scause {
    pub bits: usize,
}

impl Scause {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    // Returns the code field
    #[inline]
    pub fn code(&self) -> usize {
        self.bits & !(1 << (usize::BITS - 1))
    }

    // Trap cause
    #[inline]
    pub fn cause(&self) -> Trap {
        if self.is_interrupt() {
            Trap::Interrupt(Interrupt::from(self.code()))
        } else {
            Trap::Exception(Exception::from(self.code()))
        }
    }

    // Is trap cause an interrupt.
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        self.bits & (1 << (usize::BITS - 1)) != 0
    }

    // Is trap cause an exception.
    #[inline]
    pub fn is_exception(&self) -> bool {
        !self.is_interrupt()
    }

    #[inline]
    pub fn read() -> Self {
        let bits: usize;
        unsafe { asm!("csrr {}, scause", out(reg) bits) }

        Self { bits }
    }

    #[inline]
    pub fn write(&self) {
        unsafe { asm!("csrw scause, {}", in(reg) self.bits) }
    }

    #[inline]
    fn _clear(&self, bits: usize) {
        unsafe { asm!("csrc scause, {}", in(reg) bits) }
    }

    #[inline]
    fn _set(&self, bits: usize) {
        unsafe { asm!("csrs scause, {}", in(reg) bits) }
    }
}
