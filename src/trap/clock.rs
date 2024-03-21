use crate::{
    opensbi::Opensbi,
    riscv::{registers::sie::Sie, time},
};
use xx_mutex_lock::Mutex;

const TIMEBASE: usize = 100000;

pub fn clock_init() {
    Opensbi::sbi_set_timer(TIMEBASE);
    Sie::set_stimer();
}

pub fn clock_set_next_event() {
    Opensbi::sbi_set_timer(time::read_time() + TIMEBASE);
}

pub struct ClockCounts(Mutex<usize>);

impl ClockCounts {
    pub const fn new() -> Self {
        Self(Mutex::new(0))
    }

    pub fn add_counts(&self) -> usize {
        let mut counts = self.0.lock();
        *counts += 1;
        *counts
    }

    pub fn get_counts(&self) -> usize {
        *self.0.lock()
    }

    pub fn clear_counts(&self) {
        *self.0.lock() = 0;
    }
}
