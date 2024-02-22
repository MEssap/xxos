extern crate alloc;

use crate::mm::pagetable::PhysicalPageNumber;
use alloc::boxed::Box;

trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhysicalPageNumber>;
}

pub struct PageFrame {
    address: PhysicalPageNumber,
}

impl FrameAllocator for PageFrame {
    fn new() -> Self {
        Self { address: 0 }
    }
    fn alloc(&mut self) -> Option<PhysicalPageNumber> {
        // 直接使用Box申请一页，并得到其裸指针(在程序的生命周期中持续有效)
        let address = Box::<[u8; 4096]>::new_zeroed();
        let address = unsafe { address.assume_init() };
        let address = Box::leak(address);
        Some(address as *const _ as usize)
    }
}

impl Drop for PageFrame {
    fn drop(&mut self) {
        // 重新建立一个Box，并在作用域结束时释放
        let _: Box<[u8; 4096]> = unsafe { Box::from_raw(self.address as *mut _) };
    }
}
