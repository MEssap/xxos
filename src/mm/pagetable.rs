#![allow(unused)]
use super::{
    def::PGSZ,
    page_allocator::{alloc_page, FrameAllocator, PageFrame},
};
use crate::{
    error,
    riscv::sv39::{pteflags::*, PTE_PPN_MASK, PTE_PPN_SHIFT},
};
use alloc::{boxed::Box, vec, vec::Vec};
use core::{mem::size_of, ops::DerefMut};
use xx_mutex_lock::{Mutex, MutexGuard};
use xxos_alloc::{align_down, align_up};
use xxos_log::{error, info, warn};

#[derive(Debug)]
pub enum PageTableErr {
    AlreadyMap,
    NeverMap,
    Unknown,
    LackPageTable,
}

// PMA have 2 fields:
// 1. Page Offset field(0-11)
// 2. PPN field(12-49)
#[derive(Debug, Clone, Copy)]
pub struct PhysicalMemoryAddress(pub usize); // PMA
const PMA_OFFSET_WIDTH: u8 = 12;
const PMA_OFFSET_MASK: usize = (1 << PMA_OFFSET_WIDTH) - 1;
const PMA_PPN_SHIFT: u8 = 12;
const PMA_PPN_WIDTH: u8 = 22;
const PMA_PPN_MASK: usize = ((1 << (PMA_PPN_SHIFT + PMA_PPN_WIDTH)) - 1) ^ PMA_OFFSET_MASK;

impl From<usize> for PhysicalMemoryAddress {
    fn from(ppn: usize) -> Self {
        Self(ppn)
    }
}

impl Default for PhysicalMemoryAddress {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicalMemoryAddress {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn ppn(&self) -> PhysicalPageNumber {
        PhysicalPageNumber::from((self.0 & PMA_PPN_MASK) >> PMA_PPN_SHIFT)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PhysicalPageNumber(usize); // PPN

impl From<usize> for PhysicalPageNumber {
    fn from(ppn: usize) -> Self {
        Self(ppn)
    }
}

impl Default for PhysicalPageNumber {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicalPageNumber {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn to_pte(&self, flags: usize) -> PageTableEntry {
        PageTableEntry::from(self.0 << PTE_PPN_SHIFT | flags)
    }

    pub fn to_pma(&self) -> PhysicalMemoryAddress {
        PhysicalMemoryAddress::from(self.0 << PMA_PPN_SHIFT)
    }
}

// VMA have 2 fields:
// 1. Page Offset field(0-11)
// 2. VPN filed(12-38)
//      VPN0: 3rd pagetable index field(12-20)
//      VPN1: 2nd pagetable index field(21-29)
//      VPN2: 1st pagetable index field(30-38)
#[derive(Debug, Clone, Copy)]
pub struct VirtualMemoryAddress(usize); // VMA
const VMA_OFFSET_WIDTH: u8 = 12;
const VMA_OFFSET_MASK: usize = (1 << VMA_OFFSET_WIDTH) - 1;
const VMA_VPN_SHIFT: u8 = 12;
const VMA_VPN_WIDTH: u8 = 27;
const VMA_VPN_PART_WIDTH: u8 = 9;
const VMA_VPN_MASK: usize = ((1 << (PMA_PPN_SHIFT + PMA_PPN_WIDTH)) - 1) ^ PMA_OFFSET_MASK;

impl From<usize> for VirtualMemoryAddress {
    fn from(vpn: usize) -> Self {
        Self(vpn)
    }
}

impl Default for VirtualMemoryAddress {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtualMemoryAddress {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn offset(&self) -> usize {
        self.0 & VMA_OFFSET_MASK
    }

    pub fn vpn(&self) -> VirtualPageNumber {
        VirtualPageNumber::from((self.0 & VMA_VPN_MASK) >> VMA_VPN_SHIFT)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VirtualPageNumber(usize); // VPN

impl From<usize> for VirtualPageNumber {
    fn from(vpn: usize) -> Self {
        Self(vpn)
    }
}

impl Default for VirtualPageNumber {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtualPageNumber {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn get_part(&self, idx: usize) -> usize {
        (self.0 >> (VMA_VPN_PART_WIDTH as usize * idx)) & ((1 << VMA_VPN_PART_WIDTH) - 1)
    }
}

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

impl Default for PageTableEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl PageTableEntry {
    #[inline]
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    #[inline]
    pub fn ppn(&self) -> PhysicalPageNumber {
        PhysicalPageNumber::from((self.bits & PTE_PPN_MASK) >> PTE_PPN_SHIFT)
    }

    #[inline]
    pub fn set(&mut self, pte: usize) {
        self.bits = pte;
    }

    #[inline]
    pub fn clear(&mut self) {
        self.bits = 0;
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

#[repr(C)]
#[derive(Debug)]
pub struct PageTable {
    entrys: [PageTableEntry; PGSZ / size_of::<PageTableEntry>()],
}

#[derive(Debug)]
pub struct PageTableFrame {
    root: PhysicalMemoryAddress,
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

    pub fn root(&self) -> PhysicalMemoryAddress {
        self.root
    }

    pub fn walk(
        &mut self,
        va: VirtualMemoryAddress,
        can_alloc: bool,
    ) -> Result<&mut PageTableEntry, PageTableErr> {
        let vpn = va.vpn();
        let mut pgtb = self.root().0 as *mut PageTable;

        unsafe {
            let mut idx = 0;
            for i in (0..3).rev() {
                idx = vpn.get_part(i);

                if i != 0 {
                    if (*pgtb).entrys[idx].is_v() {
                        pgtb = ((*pgtb).entrys[idx].ppn().to_pma().0) as *mut PageTable;
                    } else if can_alloc {
                        warn!("create a new pagetable");

                        let page = alloc_page();
                        let tmp_pte = page.address().ppn().to_pte(PTE_FLAG_V | PTE_FLAG_R);

                        warn!("set pte({:#x?}) in pagetable", tmp_pte);

                        (*pgtb).entrys[idx].set(tmp_pte.bits());
                        self.frames.push(page);
                        pgtb = ((*pgtb).entrys[idx].ppn().to_pma().0) as *mut PageTable;
                    } else {
                        return Err(PageTableErr::LackPageTable);
                    }
                }
            }

            Ok(&mut (*pgtb).entrys[idx])
        }
    }

    // VPN map to PPN
    // In kernel, we use direct mapping
    pub fn map(
        &mut self,
        va: VirtualMemoryAddress,
        pa: PhysicalMemoryAddress,
        flags: usize,
    ) -> Result<&PageTableEntry, PageTableErr> {
        let ppn = pa.ppn();
        match self.walk(va, true) {
            Ok(pte) => {
                if pte.is_v() {
                    Err(PageTableErr::AlreadyMap)
                } else {
                    pte.set(ppn.to_pte(flags).bits());
                    Ok(pte)
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn unmap(&mut self, va: VirtualMemoryAddress) {
        match self.walk(va, false) {
            Ok(pte) => {
                if pte.is_v() {
                    error!("it's never map");
                } else {
                    pte.clear();
                };
            }
            Err(_) => error!("it's never map"),
        }
    }

    pub fn mappages(
        &mut self,
        va: VirtualMemoryAddress,
        pa: PhysicalMemoryAddress,
        size: usize,
        flags: usize,
    ) {
        let va = align_down!(va.0, PGSZ);
        let pa = align_down!(pa.0, PGSZ);
        let size = align_up!(size, PGSZ);

        for i in (0..size).step_by(PGSZ) {
            self.map(
                VirtualMemoryAddress::from(va + i),
                PhysicalMemoryAddress::from(pa + i),
                flags,
            );
        }
    }

    pub fn unmappages(&mut self, va: VirtualMemoryAddress, size: usize) {
        let va = VirtualMemoryAddress::from(align_down!(va.0, PGSZ));
        let size = align_up!(size, PGSZ);

        for i in (0..size).step_by(PGSZ) {
            self.unmap(va);
        }
    }
}

// TODO: Use lazylock or oncelock
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
    let va = VirtualMemoryAddress::from(pgtb.root().0);
    let flags = PTE_FLAG_V | PTE_FLAG_R;

    info!("now map pa({:#x?}) to pa({:#x?})", va, pa);

    info!("{:#x?}", pgtb.walk(va, false));

    match pgtb.map(va, pa, flags) {
        Ok(pte) => info!("pte: {:#x?}", pte),
        Err(e) => error!("{:#x?}", e),
    }
    match pgtb.map(va, pa, flags) {
        Ok(pte) => info!("pte: {:#x?}", pte),
        Err(e) => error!("{:#x?}", e),
    }

    info!("======== pagetable test end ========");
}
