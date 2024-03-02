// memory layout
pub const MEMORY_BASE: usize = 0x80000000;
pub const KERNBASE: usize = MEMORY_BASE + 0x200000; // kernel base
pub const PHYSTOP: usize = MEMORY_BASE + 128 * 1024 * 1024; // physical memory top(have 128MB)
pub const PGSZ: usize = 0x1000; // page size
pub const MAXVA: usize = 1 << (9 + 9 + 9 + 12 - 1);
pub const HEAP_TOP: usize = PHYSTOP - PGSZ * 200;
