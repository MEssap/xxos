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
use xxos_log::{error, info, warn};

#[derive(Debug)]
pub enum PageTableErr {
    AlreadyMap,
    NeverMap,
    Unknown,
    OutofRange,
}

// PMA have 2 fileds:
// 1. Page Offset field(0-11)
// 2. PPN filed(12-56)
#[derive(Debug, Clone, Copy)]
pub struct PhysicalMemoryAddress(pub usize); // PMA
const PMA_OFFSET_WIDTH: usize = 12;
const PMA_OFFSET_MASK: usize = (1 << PMA_OFFSET_WIDTH) - 1;
const PMA_PPN_SHIFT: usize = 12;
const PMA_PPN_WIDTH: usize = 22;
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

    pub fn offset(&self) -> usize {
        self.0 & PMA_OFFSET_MASK
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

    pub fn to_pma(&self, offset: usize) -> PhysicalMemoryAddress {
        PhysicalMemoryAddress::from((self.0 << PMA_PPN_SHIFT) | offset)
    }
}

// VMA have 4 fileds:
// 1. Page Offset field(0-11)
// 2. VPN0: 3rd pagetable index filed(12-20)
// 3. VPN1: 2nd pagetable index filed(21-29)
// 4. VPN2: 1st pagetable index filed(30-38)
const VPN0_SHIFT: usize = 0;
const VPN1_SHIFT: usize = 9;
const VPN2_SHIFT: usize = 18;
const VPN_PART_WIDTH: usize = 9;
const VPN_MASK: usize = (1 << (VPN_PART_WIDTH * 3)) - 1;
const VPN0_MASK: usize = (1 << VPN_PART_WIDTH) - 1;
const VPN1_MASK: usize = ((1 << (VPN1_SHIFT + VPN_PART_WIDTH)) - 1) ^ (VPN0_MASK);
const VPN2_MASK: usize = ((1 << (VPN2_SHIFT + VPN_PART_WIDTH)) - 1) ^ (VPN0_MASK | VPN1_MASK);

#[derive(Debug, Clone, Copy)]
pub struct VirtualMemoryAddress(usize); // VMA
const VMA_OFFSET_WIDTH: usize = 12;
const VMA_OFFSET_MASK: usize = (1 << PMA_OFFSET_WIDTH) - 1;
const VMA_VPN_SHIFT: usize = 12;
const VMA_VPN_WIDTH: usize = 22;
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

    pub fn value(&self) -> usize {
        self.0 & VPN_MASK
    }

    pub fn vpn0(&self) -> usize {
        self.0 & VPN0_MASK
    }

    pub fn vpn1(&self) -> usize {
        (self.0 & VPN1_MASK) >> VPN1_SHIFT
    }

    pub fn vpn2(&self) -> usize {
        (self.0 & VPN2_MASK) >> VPN2_SHIFT
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

    pub fn walk(&mut self, va: VirtualMemoryAddress) -> Result<&mut PageTableEntry, PageTableErr> {
        let vpn = va.vpn();
        let vpn0 = vpn.vpn0();
        let vpn1 = vpn.vpn1();
        let vpn2 = vpn.vpn2();
        let index: [usize; 2] = [vpn2, vpn1];
        let mut pgtb = self.root().0 as *mut PageTable;

        if vpn0 < 512 && vpn1 < 512 && vpn2 < 512 {
            info!(
                "walking at 1st Pagetable[{:#x?}], 2nd Pagetable[{:#x?}], 3rd Pagetable[{:#x?}]",
                vpn2, vpn1, vpn0
            );

            unsafe {
                for idx in index {
                    if (*pgtb).entrys[idx].is_v() {
                        if idx != vpn0 {
                            // info!("already map");
                            pgtb =
                                ((*pgtb).entrys[idx].ppn().to_pma(va.offset()).0) as *mut PageTable;
                        }
                    } else {
                        warn!("create a new pagetable");

                        let page = alloc_page();
                        let tmp_pte = page.address().ppn().to_pte(PTE_FLAG_V | PTE_FLAG_R);

                        warn!("set pte({:#x?}) in pagetable", tmp_pte);

                        (*pgtb).entrys[vpn2].set(tmp_pte.bits());
                        self.frames.push(page);
                        pgtb = ((*pgtb).entrys[vpn2].ppn().to_pma(va.offset()).0) as *mut PageTable;
                    }
                }
                Ok(&mut (*pgtb).entrys[vpn0])
            }
        } else {
            Err(PageTableErr::OutofRange)
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
        match self.walk(va) {
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
        if let Ok(pte) = self.walk(va) {
            if pte.is_v() {
                error!("it's never map");
            } else {
                pte.clear();
            }
        }
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
    let va = VirtualMemoryAddress::from(pgtb.root().0);
    let flags = PTE_FLAG_V | PTE_FLAG_R;

    info!("now map pa({:#x?}) to pa({:#x?})", va, pa);

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
