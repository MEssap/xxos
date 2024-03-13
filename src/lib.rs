#![no_std]
#![no_main]
#![feature(never_type)]
#![feature(panic_info_message)]
#![feature(new_uninit)]

pub mod console;
pub mod error;
pub mod fs;
pub mod lang_items;
pub mod mm;
pub mod opensbi;
pub mod proc;
pub mod riscv;
pub mod sched;
//pub mod task;
pub mod trap;
pub mod utils;

extern crate alloc;
