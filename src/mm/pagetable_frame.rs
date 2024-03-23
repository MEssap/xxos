use super::{
    def::PGSZ,
    page_frame::{alloc_page, PageFrame},
    pm::def::{kstack, phy_kstack, KERNEL_STACK_SIZE, MAX_PROCESS},
};
use crate::riscv::sv39::{pteflags::*, PTE_PPN_MASK, PTE_PPN_SHIFT};

use alloc::{vec, vec::Vec};
use core::{fmt::Display, mem::size_of, ops::IndexMut};
use xxos_alloc::{align_down, align_up};
use xxos_log::{error, info};
#[derive(Debug)]
pub enum PageTableErr {
    AlreadyMap,
    NeverMap,
    Unknown,
    NotFound,
}

// PMA have 2 fields:
// 1. Page Offset field(0-11)
// 2. PPN field(12-49)
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PhysicalMemoryAddress(pub usize); // PMA
const PMA_OFFSET_WIDTH: u8 = 12;
const PMA_OFFSET_MASK: usize = (1 << PMA_OFFSET_WIDTH) - 1;
const PMA_PPN_SHIFT: u8 = 12;
const PMA_PPN_WIDTH: u8 = 22;
const PMA_PPN_MASK: usize = ((1 << (PMA_PPN_SHIFT + PMA_PPN_WIDTH)) - 1) ^ PMA_OFFSET_MASK;

impl From<usize> for PhysicalMemoryAddress {
    fn from(pa: usize) -> Self {
        Self(pa)
    }
}

impl PhysicalMemoryAddress {
    pub fn to_ppn(&self) -> PhysicalPageNumber {
        ((self.0 & PMA_PPN_MASK) >> PMA_PPN_SHIFT).into()
    }
    pub fn to_pte(&self, flag: usize) -> PageTableEntry {
        let bits = (self.0 >> 12 << 10) | flag;
        PageTableEntry { bits }
    }

    pub fn get_mut_pagetable(&self) -> &'static mut PageTable {
        unsafe { (self.0 as *mut PageTable).as_mut().unwrap() }
    }
    ///
    /// # Safety
    /// self.0 not null
    pub unsafe fn get_mut<T>(&self) -> &'static mut T {
        (self.0 as *mut T).as_mut().unwrap()
    }
}

impl Display for PhysicalMemoryAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "pa: {:#x}", self.0)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PhysicalPageNumber(pub usize); // PPN

impl From<usize> for PhysicalPageNumber {
    fn from(ppn: usize) -> Self {
        Self(ppn)
    }
}

impl PhysicalPageNumber {
    pub fn to_pte(&self, flags: usize) -> PageTableEntry {
        PageTableEntry::from(self.0 << PTE_PPN_SHIFT | flags)
    }

    pub fn to_pma(&self) -> PhysicalMemoryAddress {
        PhysicalMemoryAddress::from(self.0 << PMA_PPN_SHIFT)
    }
}

/// VMA have 2 fields:
/// 1. Page Offset field(0-11)
/// 2. VPN filed(12-38)
///      VPN0: 3rd pagetable index field(12-20)
///      VPN1: 2nd pagetable index field(21-29)
///      VPN2: 1st pagetable index field(30-38)
#[derive(Debug, Clone, Copy, Default)]
pub struct VirtualMemoryAddress(pub usize); // VMA
const VMA_OFFSET_WIDTH: u8 = 12;
const VMA_OFFSET_MASK: usize = (1 << VMA_OFFSET_WIDTH) - 1;
const VMA_VPN_SHIFT: u8 = 12;
const VMA_VPN_WIDTH: u8 = 27;
const VMA_VPN_PART_WIDTH: u8 = 9;
const VMA_VPN_MASK: usize = ((1 << (VMA_VPN_SHIFT + VMA_VPN_WIDTH)) - 1) ^ VMA_OFFSET_MASK;

impl From<usize> for VirtualMemoryAddress {
    fn from(va: usize) -> Self {
        Self(va)
    }
}

impl VirtualMemoryAddress {
    pub fn vpn(&self) -> VirtualPageNumber {
        VirtualPageNumber::from((self.0 & VMA_VPN_MASK) >> VMA_VPN_SHIFT)
    }

    pub fn get_pagetable_index(&self, level: usize) -> usize {
        (self.0 >> (12 + 9 * level)) & 0x1ff
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct VirtualPageNumber(usize); // VPN

impl From<usize> for VirtualPageNumber {
    fn from(vpn: usize) -> Self {
        Self(vpn)
    }
}

impl VirtualPageNumber {
    pub fn get_pagetable_index(&self, level: usize) -> usize {
        (self.0 >> (VMA_VPN_PART_WIDTH as usize * level)) & ((1 << VMA_VPN_PART_WIDTH) - 1)
    }
}

#[repr(transparent)]
#[derive(Debug, Default)]
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
    pub fn bits(&self) -> usize {
        self.bits
    }

    pub fn get_mut_pagetable(&self) -> &'static mut PageTable {
        self.to_pma().get_mut_pagetable()
    }

    #[inline]
    pub fn to_ppn(&self) -> PhysicalPageNumber {
        ((self.bits & PTE_PPN_MASK) >> PTE_PPN_SHIFT).into()
    }

    #[inline]
    pub fn to_pma(&self) -> PhysicalMemoryAddress {
        ((self.bits >> 10) << 12).into()
    }

    #[inline]
    pub fn set(&mut self, pte: Self) {
        self.bits = pte.bits;
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
#[repr(align(0x1000))]
#[derive(Debug)]
pub struct PageTable {
    entrys: [PageTableEntry; PGSZ / size_of::<PageTableEntry>()],
}

impl PageTable {
    pub fn get_index(&mut self, index: usize) -> &mut PageTableEntry {
        self.entrys.index_mut(index)
    }
}

#[derive(Debug, Default)]
pub struct PageTableFrame {
    root: PhysicalMemoryAddress,
    frames: Vec<PageFrame>,
}

impl PageTableFrame {
    pub fn new() -> Self {
        let page = alloc_page();
        Self {
            root: page.to_pma(),
            frames: vec![page],
        }
    }

    pub fn save_page(&mut self, page: PageFrame) {
        self.frames.push(page)
    }

    pub fn root(&self) -> PhysicalMemoryAddress {
        self.root
    }

    pub fn get_mut_pagetable(&mut self) -> &'static mut PageTable {
        unsafe { (self.root().0 as *mut PageTable).as_mut().unwrap() }
    }

    pub fn map_proc_stacks(&mut self) {
        (0..MAX_PROCESS).for_each(|pid| {
            let mut va = kstack(pid);
            while va < kstack(pid) + KERNEL_STACK_SIZE {
                //这里最好直接分配N个页
                //我在这里偷懒直接使用了物理内存的最上面的一部分
                let pa = phy_kstack(pid);
                self.mappages(
                    va.into(),
                    pa.into(),
                    KERNEL_STACK_SIZE,
                    PTE_FLAG_X | PTE_FLAG_R | PTE_FLAG_W | PTE_FLAG_V,
                );
                va += KERNEL_STACK_SIZE;
            }
        });
    }

    pub fn walk(
        &mut self,
        va: VirtualMemoryAddress,
        can_alloc: bool,
    ) -> Result<&mut PageTableEntry, PageTableErr> {
        let mut pgtb = self.get_mut_pagetable();
        let mut idx = 0;
        for level in (0..3).rev() {
            idx = va.get_pagetable_index(level);
            if level == 0 {
                break;
            }
            let pte = pgtb.get_index(idx);
            if pte.is_v() {
                pgtb = pte.get_mut_pagetable();
            } else if can_alloc {
                let page = alloc_page();
                let new_page = page.to_pma().to_pte(PTE_FLAG_V);
                pte.set(new_page);
                self.save_page(page);
                pgtb = pte.get_mut_pagetable();
            } else {
                return Err(PageTableErr::NotFound);
            }
        }
        Ok(pgtb.get_index(idx))
    }

    // VPN map to PPN
    // In kernel, we use direct mapping
    pub fn map(
        &mut self,
        va: VirtualMemoryAddress,
        pa: PhysicalMemoryAddress,
        flags: usize,
    ) -> Result<&PageTableEntry, PageTableErr> {
        match self.walk(va, true)? {
            pte if pte.is_v() => Err(PageTableErr::AlreadyMap),
            pte => {
                pte.set(pa.to_pte(flags));
                Ok(pte)
            }
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
        info!("======== mappages start ========");
        let mut addr = align_down!(va.0, PGSZ);
        let last = align_down!(va.0 + size, PGSZ);
        let mut pa = pa.0;
        while addr < last {
            let Ok(_) = self.map(addr.into(), pa.into(), flags) else {
                panic!("Err")
            };
            addr += PGSZ;
            pa += PGSZ;
        }
        info!("======== mappages end ========");
    }

    pub fn unmappages(&mut self, va: VirtualMemoryAddress, size: usize) {
        let va = (align_down!(va.0, PGSZ)).into();
        let size = align_up!(size, PGSZ);
        (0..size).step_by(PGSZ).for_each(|_| {
            self.unmap(va);
        });
    }
}
