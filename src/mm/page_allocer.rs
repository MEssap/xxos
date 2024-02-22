use alloc::boxed::Box;
pub struct Page{
    address: usize
}

pub trait AllocPage {
    fn alloc_page() -> Self;
}

impl AllocPage for Page {
    fn alloc_page() -> Self {
        let address = Box::<[u8;4096]>::new_zeroed();
        let address = unsafe {address.assume_init()};
        let address = Box::leak(address);
        Self { address: address as *const _ as usize }
    }
}

impl Drop for Page {
    fn drop(&mut self) {
        let address = self.address;
        let ptr: Box<[u8;4096]> = unsafe { Box::from_raw(address as *mut _) };
        drop(ptr)
    }
}

