#![no_main]
#![no_std]
use core::arch::global_asm;
use xxos::println;

global_asm!(include_str!("entry.s"));


#[no_mangle]
fn main() {
    clear_bss();
    println!("hello world.");

    panic!()
}

fn clear_bss() {
    extern "C" {
        fn bss_start();
        fn bss_end();
    }
    (bss_start as usize..bss_end as usize)
        .for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
