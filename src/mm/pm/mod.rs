pub mod def;
use xxos_alloc::LockedSlab;
use xxos_log::info;

use crate::mm::pm::def::HEAP_TOP;

// 定义新的分配器
#[global_allocator]
static ALLOCATOR: LockedSlab = LockedSlab::new_uninit();

pub fn heap_init() {
    extern "C" {
        fn ekernel();
    }
    let btm = ekernel as usize;
    let top = HEAP_TOP;

    info!("memory bottom is {:#x}, memory top is {:#x} ", btm, top);
    ALLOCATOR.init(btm, top);
}
