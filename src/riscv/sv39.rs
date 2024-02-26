// SV39 entry have 2 fileds
// 1. Page Table Entry Flags filed(0-9)
// 2. Physical Page Number filed(10-53)
// and reserved 10 bits
pub const PTE_FLAGS_SHIFT: u8 = 0;
pub const PTE_FLAGS_WIDTH: u8 = 10;
pub const PTE_FLAGS_MASK: usize = (1 << PTE_FLAGS_WIDTH) - 1;

pub const PTE_PPN_SHIFT: u8 = 10;
pub const PTE_PPN_WIDTH: u8 = 44;
pub const PTE_PPN_MASK: usize = ((1 << (PTE_PPN_SHIFT + PTE_PPN_WIDTH + 1)) - 1) ^ PTE_FLAGS_MASK;

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
        V = PTE_FLAG_V as isize,
        R = PTE_FLAG_R as isize,
        W = PTE_FLAG_W as isize,
        X = PTE_FLAG_X as isize,
        U = PTE_FLAG_U as isize,
        G = PTE_FLAG_G as isize,
        A = PTE_FLAG_A as isize,
        D = PTE_FLAG_D as isize,
        RW = (PTE_FLAG_R + PTE_FLAG_W) as isize,
        RWX = (PTE_FLAG_R + PTE_FLAG_W + PTE_FLAG_X) as isize,
        // 如果使用enum则需要穷举完存在的flag搭配?
        // 有无更好的方式能够一次性检验多个flags?
    }
}
