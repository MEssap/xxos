use core::arch::asm;

#[derive(Debug, Default, Clone)]
pub struct Sepc {
    bits: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrapMode {
    Direct = 0,
    Vectored = 1,
}

impl Sepc {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    #[inline]
    pub fn read() -> Self {
        let bits: usize;
        unsafe { asm!("csrr {}, sepc", out(reg) bits) }
        Self { bits }
    }

    #[inline]
    pub fn address(&self) -> usize {
        self.bits & !((1 << 2) - 1)
    }

    #[inline]
    pub fn set_address(&mut self, addr: usize) {
        let mode = self.mode();
        self.bits = addr + mode as usize;
    }

    #[inline]
    pub fn mode(&self) -> TrapMode {
        match self.bits & ((1 << 2) - 1) {
            0 => TrapMode::Direct,
            1 => TrapMode::Vectored,
            _ => {
                panic!("unknow trapmode");
            }
        }
    }

    #[inline]
    pub fn set_mode(&mut self, mode: TrapMode) {
        let addr = self.address();
        self.bits = addr + mode as usize;
    }

    #[inline]
    pub fn write(&self) {
        unsafe { asm!("csrw sepc, {}", in(reg) self.bits) };
    }
}
