#![allow(unused)]

use super::pagetable::PageTableEntry;

// PMA have 2 fileds:
// 1. Page Offset field(0-11)
// 2. PPN filed(12-56)
pub type PhysicalMemoryAddress = usize;
pub struct PhysicalPageNumber(pub usize);

impl PhysicalPageNumber {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn get_pte_array(&self) -> &'static mut [PageTableEntry] {
        let pa: PhysicalMemoryAddress = self.0;
        unsafe { core::slice::from_raw_parts_mut(pa as *mut PageTableEntry, 512) }
    }

    pub fn get_bytes_array(&self) -> &'static mut [u8] {
        let pa: PhysicalMemoryAddress = self.0;
        unsafe { core::slice::from_raw_parts_mut(pa as *mut u8, 4096) }
    }

    pub fn get_mut<T>(&self) -> &'static mut T {
        let pa: PhysicalMemoryAddress = self.0;
        unsafe { (pa as *mut T).as_mut().unwrap() }
    }
}

// VMA have 4 fileds:
// 1. Page Offset field(0-11)
// 2. VPN0: 3rd pagetable index filed(12-20)
// 3. VPN1: 2nd pagetable index filed(21-29)
// 4. VPN2: 1st pagetable index filed(30-38)
pub type VirtualMemoryAddress = usize;
pub struct VirtualPageNumber(pub usize);

impl VirtualPageNumber {
    pub fn new() -> Self {
        Self(0)
    }
}
