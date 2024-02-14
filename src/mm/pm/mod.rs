pub mod def;
use xxos_alloc::LockedSlab;
use xxos_log::info;

use crate::mm::pm::def::{PGSZ, PHYSTOP};

#[global_allocator]
static ALLOCATOR: LockedSlab = LockedSlab::new_uninit();

pub fn heap_init() {
    extern "C" {
        fn ekernel();
    }
    let btm = ekernel as usize;
    let top = PHYSTOP - PGSZ * 100;

    info!("memory bottom is {:#x}, memory top is {:#x} ", btm, top);
    ALLOCATOR.init(btm, top);
}
