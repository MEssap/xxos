extern crate alloc;

use crate::mm::pagetable::PhysicalPageNumber;
use alloc::boxed::Box;
use xxos_log::error;

use super::pagetable::PhysicalMemoryAddress;

pub trait FrameAllocator {
    fn alloc() -> Self;
}

#[derive(Debug)]
pub struct PageFrame {
    address: PhysicalMemoryAddress,
}

impl FrameAllocator for PageFrame {
    fn alloc() -> Self {
        // 直接使用Box申请一页，并得到其裸指针(在程序的生命周期中持续有效)
        let address = Box::<[u8; 4096]>::new_zeroed();
        let address = unsafe { address.assume_init() };
        let address = Box::leak(address);
        Self {
            address: PhysicalMemoryAddress::from(address as *const _ as usize),
        }
    }
}

impl From<PhysicalMemoryAddress> for PageFrame {
    fn from(ppn: PhysicalMemoryAddress) -> Self {
        Self { address: ppn }
    }
}

impl Drop for PageFrame {
    fn drop(&mut self) {
        // 重新建立一个Box，并在作用域结束时释放
        let _: Box<[u8; 4096]> = unsafe { Box::from_raw(self.address.0 as *mut _) };
    }
}

impl PageFrame {
    pub fn address(&self) -> PhysicalMemoryAddress {
        self.address
    }
}

pub fn alloc_page() -> PageFrame {
    PageFrame::alloc()
}
