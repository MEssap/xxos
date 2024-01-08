pub const SBI_SET_TIMER: usize = 0;
pub const SBI_CONSOLE_PUTCHAR: usize = 1;
pub const SBI_CONSOLE_GETCHAR: usize = 2;
pub const SBI_CLEAR_IPI: usize = 3;
pub const SBI_SEND_IPI: usize = 4;
pub const SBI_REMOTE_FENCE_I: usize = 5;
pub const SBI_REMOTE_SFENCE_VMA: usize = 6;
pub const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
pub const SBI_SHUTDOWN: usize = 8;

pub const SBI_EXT_HSM: usize = 0x48534D;
pub const SBI_EXT_HSM_HART_START: usize = 0;
pub const SBI_EXT_HSM_HART_STOP: usize = 1;
pub const SBI_EXT_HSM_HART_GET_STATUS: usize = 2;
pub const SBI_EXT_HSM_HART_SUSPEND: usize = 3;

pub const N_HART: usize = 3;
