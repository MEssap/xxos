use core::arch::asm;

/// regiter sstatus(Supervisor Status Register)
#[derive(Debug, Default, Clone)]
pub struct Sstatus {
    bits: usize,
}

// Supervisor Previous Privilege Mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPP {
    Supervisor = 0b01,
    User = 0b00,
}

const SIE: usize = 1 << 1;
const SPP: usize = 1 << 8;
const SUM: usize = 1 << 18;
const MXR: usize = 1 << 19;
const SPIE: usize = 1 << 5; // Supervisor Previous Interrupt Enable
impl Sstatus {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn bits(&self) -> usize {
        self.bits
    }

    pub fn set(&mut self, bits: usize) {
        self.bits = bits;
    }
    // Supervisor Interrupt Enable
    #[inline]
    pub fn sie(&self) -> bool {
        self.bits & SIE != 0
    }

    pub fn set_sie(&mut self) {
        self.bits |= SIE;
    }

    // Supervisor Previous Privilege Mode
    #[inline]
    pub fn spp(&self) -> SPP {
        if self.bits & SPP != 0 {
            SPP::Supervisor
        } else {
            SPP::User
        }
    }

    pub fn set_spp(spp: SPP) {
        match spp {
            SPP::User => Self::_clear(SPP),
            SPP::Supervisor => Self::_set(SPP),
        }
    }

    // Permit Supervisor User Memory access
    #[inline]
    pub fn sum(&self) -> bool {
        self.bits & SUM != 0
    }
    #[inline]
    pub fn set_spie() {
        Self::_set(SPIE);
    }

    // Make eXecutable Readable
    #[inline]
    pub fn mxr(&self) -> bool {
        self.bits & MXR != 0
    }

    #[inline]
    pub fn read() -> Self {
        let bits: usize;
        unsafe { asm!("csrr {}, sstatus", out(reg) bits) }
        Self { bits }
    }

    #[inline]
    pub fn write(&self) {
        unsafe { asm!("csrw sstatus, {}", in(reg) self.bits) }
    }

    #[inline]
    pub fn clear_sie() {
        Self::_clear(SIE);
    }

    #[inline]
    fn _clear(bits: usize) {
        unsafe { asm!("csrc sstatus, {}", in(reg) bits) }
    }

    #[inline]
    fn _set(bits: usize) {
        unsafe { asm!("csrs sstatus, {}", in(reg) bits) }
    }
}
#[inline]
pub fn intr_off() {
    Sstatus::clear_sie();
}
#[inline]
pub fn intr_on() {
    Sstatus::_set(SIE);
}
