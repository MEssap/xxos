#![allow(unused)]
use alloc::{boxed::Box, vec, vec::Vec};
use core::{mem::size_of, ops::DerefMut};
use xx_mutex_lock::{Mutex, MutexGuard};
use xxos_log::{error, info, warn};

use super::{
    def::PGSZ,
    page_allocator::{alloc_page, FrameAllocator, PageFrame},
};
use crate::{
    error,
    riscv::sv39::{pteflags::*, PPN_MASK, PPN_OFFSET},
};

// PMA have 2 fileds:
// 1. Page Offset field(0-11)
// 2. PPN filed(12-56)
pub type PhysicalMemoryAddress = usize; // PMA
pub type PhysicalPageNumber = usize; // PPN

// VMA have 4 fileds:
// 1. Page Offset field(0-11)
// 2. VPN0: 3rd pagetable index filed(12-20)
// 3. VPN1: 2nd pagetable index filed(21-29)
// 4. VPN2: 1st pagetable index filed(30-38)
const VPN0_OFFSET: usize = 12;
const VPN1_OFFSET: usize = 21;
const VPN2_OFFSET: usize = 30;
const VPN_PART_WIDTH: usize = 9;
const VPN0_MASK: usize = ((1 << (VPN0_OFFSET + VPN_PART_WIDTH)) - 1) ^ ((1 << VPN0_OFFSET) - 1);
const VPN1_MASK: usize = ((1 << (VPN1_OFFSET + VPN_PART_WIDTH)) - 1) ^ ((1 << VPN1_OFFSET) - 1);
const VPN2_MASK: usize = ((1 << (VPN2_OFFSET + VPN_PART_WIDTH)) - 1) ^ ((1 << VPN2_OFFSET) - 1);
pub type VirtualMemoryAddress = usize; // VMA
pub type VirtualPageNumber = usize; // VPN

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PageTableEntry {
    bits: usize,
}

impl From<usize> for PageTableEntry {
    fn from(pte: usize) -> Self {
        Self { bits: pte }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PageTable {
    entrys: [PageTableEntry; PGSZ / size_of::<PageTableEntry>()],
}

impl From<PhysicalPageNumber> for PageTable {
    fn from(ppn: PhysicalPageNumber) -> Self {
        unsafe { *(ppn as *mut PageTable) }
    }
}

impl From<PageFrame> for PageTable {
    fn from(page: PageFrame) -> Self {
        info!("from PageFrame {:#x} into PageTable", page.address());
        unsafe { *(page.address() as *mut PageTable) }
    }
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

    pub fn walk(&mut self, vpn: usize, alloc: bool, perm: usize) -> Result<(), ()> {
        let mut pagetable = PageTable::from(self.root);

        for level in (1..3).rev() {
            let mut pte = pagetable.entrys[0];

            if pte.is_v() {
                pagetable = PageTable::from(pte.ppn());
            } else {
                if !alloc {
                    return Err(());
                } else {
                    let page = alloc_page();
                    pte.set(page.address() << PPN_OFFSET);
                    pagetable = PageTable::from(page.address());
                    self.frames.push(page);
                }
            }
        }

        Ok(())
    }

    pub fn mappages(
        &mut self,
        vpn: VirtualPageNumber,
        ppn: PhysicalPageNumber,
        perm: usize,
    ) -> Result<(), ()> {
        loop {
            match self.walk(vpn, true, perm) {
                Ok(pte) => {
                    info!("run here {:#x?}", pte);
                }
                Err(e) => {
                    break Err(e);
                }
            }
        }
    }

    // VPN map to PPN
    // In kernel, we use direct mapping
    pub fn map(
        &mut self,
        vpn: VirtualPageNumber,
        ppn: PhysicalPageNumber,
        flags: usize,
    ) -> Result<(), ()> {
        info!("start to map");

        let vpn0 = (vpn & VPN0_MASK) * 4;
        let vpn1 = (vpn & VPN1_MASK) * 4;
        let vpn2 = (vpn & VPN2_MASK) * 4;
        let mut pgtb = PageTable::from(self.root());

        // TODO: get index from vpnX
        let idx0 = 0; // from vpn2
        let idx1 = 0; // from vpn1
        let idx2 = 0; // from vpn0

        // TODO: create `walk` function
        // walk in 1st pagetable
        if pgtb.entrys[idx0].is_v() {
            //pgtb = PageTable::from(pgtb.entrys[idx0].ppn());
        } else {
            warn!("this vpn never map in 1st pagetable");
            let page = alloc_page();
            pgtb.entrys[idx0].set(page.address() << PPN_OFFSET | PTE_FLAG_V);
            self.frames.push(page);
            //pgtb = PageTable::from(pgtb.entrys[idx0].ppn());
        }

        // walk in 2st pagetable
        if pgtb.entrys[idx1].is_v() {
            error!("pte: {:#x?}", pgtb.entrys[idx1]);
            //pgtb = PageTable::from(pgtb.entrys[idx1].ppn());
        } else {
            warn!("this vpn never map in 2nd pagetable");
            let page = alloc_page();
            pgtb.entrys[idx1].set(page.address() << PPN_OFFSET | PTE_FLAG_V);
            self.frames.push(page);
            //pgtb = PageTable::from(pgtb.entrys[idx1].ppn());
        }

        // walk in 3st pagetable and map vpn to ppn
        if pgtb.entrys[idx2].is_v() {
            error!("already map");
            Err(())
        } else {
            warn!("this vpn never map in 3rd pagetable");
            let page = alloc_page();
            pgtb.entrys[idx2].set(page.address() << PPN_OFFSET | PTE_FLAG_V);
            self.frames.push(page);

            Ok(())
        }
    }

    pub fn unmap(&mut self, vpn: VirtualPageNumber) {}
}

impl PageTableEntry {
    #[inline]
    pub fn ppn(&self) -> PhysicalPageNumber {
        (self.bits & PPN_MASK) >> PPN_OFFSET
    }

    #[inline]
    pub fn set(&mut self, ppn: PhysicalPageNumber) {
        self.bits = ppn << PPN_OFFSET;
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
    let vpn0: usize = 1 << VPN0_OFFSET;
    let vpn1: usize = 1 << VPN1_OFFSET;
    let vpn2: usize = 1 << VPN2_OFFSET;
    let offset = 0xaaa;
    let flags = PTE_FLAG_V | PTE_FLAG_R;
    let vpn = vpn0 + vpn1 + vpn2 + offset;

    info!("vpn0: {:#x}", vpn0);
    info!("vpn1: {:#x}", vpn1);
    info!("vpn2: {:#x}", vpn2);
    info!("vpn: {:#x}", vpn);

    info!("pagetableframe created: {:#x?}", pgtb);

    //pgtb.mappages(vpn, vpn, flags);
    // if use LockedPageTableFrame
    //pgtb.0.lock().map(vpn, vpn, flags);
    pgtb.map(vpn, vpn, flags);

    info!("pagetable: {:#x?}", unsafe {
        *(pgtb.root() as *mut PageTable)
    });

    info!("======== pagetable test end ========");
}
