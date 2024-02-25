#![allow(unused)]
use super::{
    def::PGSZ,
    page_allocator::{alloc_page, FrameAllocator, PageFrame},
};
use crate::{
    error,
    riscv::sv39::{pteflags::*, PTE_PPN_MASK, PTE_PPN_OFFSET},
};
use alloc::{boxed::Box, vec, vec::Vec};
use core::{mem::size_of, ops::DerefMut};
use xx_mutex_lock::{Mutex, MutexGuard};
use xxos_log::{error, info, warn};

#[derive(Debug)]
pub enum PageTableErr {
    AlreadyMap,
}

// PMA have 2 fileds:
// 1. Page Offset field(0-11)
// 2. PPN filed(12-56)
pub type PhysicalMemoryAddress = usize; // PMA
pub type PhysicalPageNumber = usize; // PPN
const PPN_OFFSET: usize = 12;

// VMA have 4 fileds:
// 1. Page Offset field(0-11)
// 2. VPN0: 3rd pagetable index filed(12-20)
// 3. VPN1: 2nd pagetable index filed(21-29)
// 4. VPN2: 1st pagetable index filed(30-38)
const VPN1_OFFSET: usize = 9;
const VPN2_OFFSET: usize = 18;
const VPN_PART_WIDTH: usize = 9;
const VPN0_MASK: usize = (1 << (VPN_PART_WIDTH)) - 1;
const VPN1_MASK: usize = ((1 << (VPN1_OFFSET + VPN_PART_WIDTH)) - 1) ^ (VPN0_MASK);
const VPN2_MASK: usize = ((1 << (VPN2_OFFSET + VPN_PART_WIDTH)) - 1) ^ (VPN0_MASK | VPN1_MASK);
pub type VirtualMemoryAddress = usize; // VMA
pub type VirtualPageNumber = usize; // VPN

#[repr(C)]
#[derive(Debug)]
pub struct PageTableEntry {
    bits: usize,
}

impl From<usize> for PageTableEntry {
    fn from(pte: usize) -> Self {
        Self { bits: pte }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PageTable {
    entrys: [PageTableEntry; PGSZ / size_of::<PageTableEntry>()],
}

#[derive(Debug)]
pub struct PageTableFrame {
    root: PhysicalPageNumber,
    frames: Vec<PageFrame>,
}

impl Default for PageTableFrame {
    fn default() -> Self {
        Self::new()
    }
}

impl PageTableFrame {
    pub fn new() -> Self {
        let page = alloc_page();
        Self {
            root: page.address(),
            frames: vec![page],
        }
    }

    pub fn root(&self) -> PhysicalPageNumber {
        self.root
    }

    // VPN map to PPN
    // In kernel, we use direct mapping
    pub fn map(
        &mut self,
        vpn: VirtualPageNumber,
        ppn: PhysicalPageNumber,
        flags: usize,
    ) -> Result<&PageTableEntry, PageTableErr> {
        info!("vpn: {:#x?} ppn: {:#x?}", vpn, ppn);

        let vpn0 = (vpn & VPN0_MASK);
        let vpn1 = (vpn & VPN1_MASK) >> VPN1_OFFSET;
        let vpn2 = (vpn & VPN2_MASK) >> VPN2_OFFSET;
        let mut pgtb = self.root() as *mut PageTable;

        info!("vpn0: {:#x?} vpn1: {:#x?} vpn2: {:#x?}", vpn0, vpn1, vpn2);

        // TODO: create `walk` function
        unsafe {
            // walk in 1st pagetable
            if (*pgtb).entrys[vpn2].is_v() {
                info!("already map in 1st pagetable");
                pgtb = ((*pgtb).entrys[vpn2].ppn() << PPN_OFFSET) as *mut PageTable;
            } else {
                warn!("this vpn never map in 1st pagetable");
                let page = alloc_page();
                let pte = (page.address() >> PPN_OFFSET) << PTE_PPN_OFFSET | flags;
                warn!("set pte({:#x?}) in 1st pagetable {} entry", pte, vpn2);
                (*pgtb).entrys[vpn2].set(pte);
                self.frames.push(page);
                pgtb = ((*pgtb).entrys[vpn2].ppn() << PPN_OFFSET) as *mut PageTable;
            }

            // walk in 2nd pagetable
            if (*pgtb).entrys[vpn1].is_v() {
                info!("already map in 2nd pagetable");
                pgtb = ((*pgtb).entrys[vpn1].ppn() << PPN_OFFSET) as *mut PageTable;
            } else {
                warn!("this vpn never map in 2nd pagetable");
                let page = alloc_page();
                let pte = (page.address() >> PPN_OFFSET) << PTE_PPN_OFFSET | flags;
                warn!("set pte({:#x?}) in 2nd pagetable {} entry", pte, vpn1);
                (*pgtb).entrys[vpn1].set(pte);
                self.frames.push(page);
                pgtb = ((*pgtb).entrys[vpn1].ppn() << PPN_OFFSET) as *mut PageTable;
            }

            // walk in 3rd pagetable and map vpn to ppn
            if (*pgtb).entrys[vpn0].is_v() {
                error!("already map in 3rd pagetable");
                Err(PageTableErr::AlreadyMap)
            } else {
                warn!("this vpn never map in 3rd pagetable");
                let page = alloc_page();
                let pte = (page.address() >> PPN_OFFSET) << PTE_PPN_OFFSET | flags;
                warn!("set pte({:#x?}) in 3rd pagetable {} entry", pte, vpn0);
                (*pgtb).entrys[vpn0].set(pte);
                self.frames.push(page);

                Ok(&(*pgtb).entrys[vpn0])
            }
        }
    }

    pub fn unmap(&mut self, vpn: VirtualPageNumber) {}
}

impl PageTableEntry {
    #[inline]
    pub fn ppn(&self) -> PhysicalPageNumber {
        (self.bits & PTE_PPN_MASK) >> PTE_PPN_OFFSET
    }

    #[inline]
    pub fn set(&mut self, ppn: PhysicalPageNumber) {
        self.bits = ppn;
    }

    #[inline]
    pub fn check_flags(&self, flags: PTEFlags) -> bool {
        self.bits & (flags as usize) != 0
    }

    #[inline]
    pub fn is_v(&self) -> bool {
        self.bits & PTE_FLAG_V != 0
    }

    #[inline]
    pub fn is_r(&self) -> bool {
        self.bits & PTE_FLAG_R != 0
    }

    #[inline]
    pub fn is_w(&self) -> bool {
        self.bits & PTE_FLAG_W != 0
    }

    #[inline]
    pub fn is_x(&self) -> bool {
        self.bits & PTE_FLAG_X != 0
    }

    #[inline]
    pub fn is_u(&self) -> bool {
        self.bits & PTE_FLAG_U != 0
    }

    #[inline]
    pub fn is_g(&self) -> bool {
        self.bits & PTE_FLAG_G != 0
    }

    #[inline]
    pub fn is_a(&self) -> bool {
        self.bits & PTE_FLAG_A != 0
    }

    #[inline]
    pub fn is_d(&self) -> bool {
        self.bits & PTE_FLAG_D != 0
    }
}

pub struct LockedPageTableFrame(Mutex<PageTableFrame>);

impl Default for LockedPageTableFrame {
    fn default() -> Self {
        Self::new()
    }
}

impl LockedPageTableFrame {
    pub fn new() -> Self {
        Self(Mutex::new(PageTableFrame::new()))
    }
}

pub fn pgtb_test() {
    info!("======== pagetable test start ========");
    //let mut pgtb = LockedPageTableFrame::new();
    let mut pgtb = PageTableFrame::new();

    info!("pagetableframe created: {:#x?}", pgtb);

    let pa = pgtb.root();
    let va = pgtb.root();

    let ppn = pa >> PPN_OFFSET;
    let offset = ppn & ((1 << 12) - 1);
    let vpn = ppn;
    let flags = PTE_FLAG_V | PTE_FLAG_R;

    info!("now map vpn({:#x?}) to ppn({:#x?})", vpn, ppn);

    match pgtb.map(vpn, ppn, flags) {
        Ok(pte) => info!("pte: {:#x?}", pte),
        Err(e) => error!("{:#x?}", e),
    }
    match pgtb.map(vpn, ppn, flags) {
        Ok(pte) => info!("pte: {:#x?}", pte),
        Err(e) => error!("{:#x?}", e),
    }

    info!("======== pagetable test end ========");
}
