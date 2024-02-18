pub const PTE_PPN_OFFSET: usize = 10;
pub const PTE_PPN_WIDTH: usize = 44;
pub const PTE_PPN: usize =
    ((1 << (PTE_PPN_OFFSET + PTE_PPN_WIDTH)) - 1) ^ ((1 << PTE_PPN_OFFSET) - 1);
