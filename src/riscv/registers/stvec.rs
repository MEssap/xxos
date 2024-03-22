use core::arch::asm;

// Supervisor Trap-Vector Base Address
pub struct Stvec {
    bits: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrapMode {
    Direct = 0,
    Vectored = 1,
}

impl From<usize> for TrapMode {
    fn from(value: usize) -> Self {
        match value {
            0 => TrapMode::Direct,
            1 => TrapMode::Vectored,
            _ => panic!("Unknow trapmode"),
        }
    }
}

impl Stvec {
    #[inline]
    pub fn read() -> Self {
        let bits: usize;
        unsafe { asm!("csrr {}, stvec", out(reg) bits) }
        Self { bits }
    }

    #[inline]
    pub fn write(addr: usize, mode: TrapMode) {
        unsafe { asm!("csrw stvec, {}", in(reg) addr + mode as usize) };
    }

    #[inline]
    pub fn mode(&self) -> TrapMode {
        match self.bits & ((1 << 2) - 1) {
            0 => TrapMode::Direct,
            1 => TrapMode::Vectored,
            _ => panic!("Unknow trapmode"),
        }
    }

    #[inline]
    pub fn set_mode(&mut self, mode: TrapMode) {
        self.bits = (self.bits & !((1 << 2) - 1)) + mode as usize;
    }
}
