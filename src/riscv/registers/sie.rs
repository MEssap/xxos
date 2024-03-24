use core::arch::asm;

const SEIE: usize = 1 << 9; // external
const STIE: usize = 1 << 5; // timer
const SSIE: usize = 1 << 1; // software

pub struct Sie;

impl Sie {
    #[inline]
    pub fn set_sext() {
        Self::_set(SEIE);
    }

    #[inline]
    pub fn clear_sext() {
        Self::_clear(SEIE);
    }

    #[inline]
    pub fn set_stimer() {
        Self::_set(STIE);
    }

    #[inline]
    pub fn clear_stimer() {
        Self::_clear(STIE);
    }

    #[inline]
    pub fn set_ssoft() {
        Self::_set(SSIE);
    }

    #[inline]
    pub fn clear_ssoft() {
        Self::_clear(SSIE);
    }

    #[inline]
    fn _clear(bits: usize) {
        unsafe { asm!("csrc sie, {}", in(reg) bits) }
    }

    #[inline]
    fn _set(bits: usize) {
        unsafe { asm!("csrs sie, {}", in(reg) bits) }
    }
}
