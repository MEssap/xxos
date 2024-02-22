use alloc::boxed::Box;

use crate::mm::pagetable::PageTable;

// Kernel Virtual Memory
pub struct Kvm {
    root_pgtb: Box<PageTable>,
}
