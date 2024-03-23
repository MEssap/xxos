// memory layout
pub const MEMORY_BASE: usize = 0x80000000;
pub const KERNBASE: usize = MEMORY_BASE + 0x200000; // kernel base
pub const PHYSTOP: usize = MEMORY_BASE + 128 * 1024 * 1024; // physical memory top(have 128MB)
pub const PGSZ: usize = 0x1000; // page size
pub const MAXVA: usize = 1 << (9 + 9 + 9 + 12 - 1);
pub const HEAP_TOP: usize = PHYSTOP - PGSZ * 200;
pub const TRAMPOLINE: usize = MAXVA - PGSZ;
pub const TRAPFRAME: usize = TRAMPOLINE - PGSZ;
pub const KERNEL_STACK_SIZE: usize = PGSZ * 3;

pub const MAX_PROCESS: usize = 32;
#[inline]
pub fn kstack(pid: usize) -> usize {
    //n+1
    TRAPFRAME - PGSZ - (pid + 1) * ((KERNEL_STACK_SIZE / PGSZ + 1) * PGSZ)
}

#[inline]
pub fn phy_kstack(pid: usize) -> usize {
    //n+1
    PHYSTOP - (10 * PGSZ) - (pid + 1) * ((KERNEL_STACK_SIZE / PGSZ + 1) * PGSZ)
}
