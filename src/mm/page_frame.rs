extern crate alloc;
use super::{def::PGSZ, pagetable_frame::PhysicalMemoryAddress};
use alloc::boxed::Box;

#[derive(Debug, Default)]
pub struct PageFrame {
    address: PhysicalMemoryAddress,
}

impl PageFrame {
    fn alloc() -> Self {
        // 直接使用Box申请一页，并得到其裸指针(在程序的生命周期中持续有效)
        let address = Box::<[u8; 4096]>::new_zeroed();
        let address = unsafe { address.assume_init() };
        let address = Box::leak(address);
        Self {
            address: (address as *const _ as usize).into(),
        }
    }
}

impl From<PhysicalMemoryAddress> for PageFrame {
    fn from(pma: PhysicalMemoryAddress) -> Self {
        Self { address: pma }
    }
}

impl Drop for PageFrame {
    fn drop(&mut self) {
        // 重新建立一个Box，并在作用域结束时释放
        let _a: Box<[u8; PGSZ]> = unsafe { Box::from_raw(self.address.0 as *mut _) };
    }
}

impl PageFrame {
    pub fn to_pma(&self) -> PhysicalMemoryAddress {
        self.address
    }
}

pub fn alloc_page() -> PageFrame {
    PageFrame::alloc()
}
