/// regiter sstatus(Supervisor Status Register)
pub struct Sstatus {
    bits: usize,
}

// Supervisor Previous Privilege Mode
pub enum SPP {
    Machine = 0b11,
    Supervisor = 0b01,
    User = 0b00,
}

impl Sstatus {
    pub const fn new() -> Self {
        Self { bits: 0 }
    }

    // Supervisor Interrupt Enable
    #[inline]
    pub fn sie(&self) -> bool {
        self.bits & (1 << 1) != 0
    }

    // Supervisor Previous Privilege Mode
    #[inline]
    pub fn spp(&self) -> SPP {
        if self.bits & (1 << 8) != 0 {
            SPP::Supervisor
        } else {
            SPP::User
        }
    }

    // Permit Supervisor User Memory access
    #[inline]
    pub fn sum(&self) -> bool {
        self.bits & (1 << 18) != 0
    }

    // Make eXecutable Readable
    #[inline]
    pub fn mxr(&self) -> bool {
        self.bits & (1 << 19) != 0
    }
}

pub mod sstatus {
    use super::Sstatus;
    use core::arch::asm;

    #[inline]
    pub fn read() -> Sstatus {
        let mut bits = 0;
        unsafe { asm!("csrr {}, sstatus", out(reg) bits) }
        Sstatus { bits }
    }

    #[inline]
    pub fn write(bits: usize) {
        unsafe { asm!("csrw sstatus, {}", in(reg) bits) }
    }

    #[inline]
    pub unsafe fn clear(bits: usize) {
        asm!("csrc sstatus, {}", in(reg) bits);
    }
}