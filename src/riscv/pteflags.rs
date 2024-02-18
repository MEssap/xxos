pub enum PteFlags {
    V = 1,
    R = 1 << 1,
    W = 1 << 2,
    X = 1 << 3,
    U = 1 << 4,
    G = 1 << 5,
    A = 1 << 6,
    D = 1 << 7,
}

pub const PTE_FLAG_V: usize = 1;
pub const PTE_FLAG_R: usize = 1 << 1;
pub const PTE_FLAG_W: usize = 1 << 2;
pub const PTE_FLAG_X: usize = 1 << 3;
pub const PTE_FLAG_U: usize = 1 << 4;
pub const PTE_FLAG_G: usize = 1 << 5;
pub const PTE_FLAG_A: usize = 1 << 6;
pub const PTE_FLAG_D: usize = 1 << 7;
