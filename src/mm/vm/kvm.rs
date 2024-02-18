use crate::mm::pagetable::PageTable;

// Kernel Virtual Memory
pub struct Kvm {
    pagetable: *mut PageTable,
}
