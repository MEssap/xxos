// SV39 entry have 2 fileds
// 1. Page Table Entry Flags filed(0-9)
// 2. Physical Page Number filed(10-53)
// and reserved 10 bits
pub const PTE_FLAGS_OFFSET: u8 = 0;
pub const PTE_FLAGS_WIDTH: u8 = 10;
pub const PTE_FLAGS_MASK: usize = (1 << PTE_FLAGS_WIDTH) - 1;

pub const PPN_OFFSET: u8 = 10;
pub const PPN_WIDTH: u8 = 44;
pub const PPN_MASK: usize = ((1 << (PPN_OFFSET + PPN_WIDTH)) - 1) ^ PTE_FLAGS_MASK;

pub mod pteflags {
    pub const PTE_FLAG_V: usize = 1;
    pub const PTE_FLAG_R: usize = 1 << 1;
    pub const PTE_FLAG_W: usize = 1 << 2;
    pub const PTE_FLAG_X: usize = 1 << 3;
    pub const PTE_FLAG_U: usize = 1 << 4;
    pub const PTE_FLAG_G: usize = 1 << 5;
    pub const PTE_FLAG_A: usize = 1 << 6;
    pub const PTE_FLAG_D: usize = 1 << 7;
    // reserved 2 bits

    pub enum PTEFlags {
        V = 1,
        R = 1 << 1,
        W = 1 << 2,
        X = 1 << 3,
        U = 1 << 4,
        G = 1 << 5,
        A = 1 << 6,
        D = 1 << 7,
    }
}
