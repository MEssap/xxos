#![no_main]
#![no_std]
use core::arch::global_asm;
use core::sync::atomic::{AtomicBool, Ordering};
use alloc::vec;
use xxos::console::Log;
use xxos::mm;
use xxos::opensbi::thread_start;
use xxos::println;
static STARTED: AtomicBool = AtomicBool::new(false);
extern crate alloc;
global_asm!(include_str!("entry.s"));

#[no_mangle]
fn main() {
    thread_start();
    let thread_id = xxos::opensbi::r_tp();
    if thread_id == 0 {
        clear_bss();
        xxos_log::init_log(&Log, xxos_log::Level::INFO);
        mm::pm::heap_init();
        let a = vec![1,2,3,4];
        println!("{:?}",a);
        println!("Thread {} start !!!", thread_id);
        STARTED.store(true, Ordering::SeqCst);
    } else {
        loop {
            if STARTED.load(Ordering::SeqCst) {
                break;
            }
        }
        println!("Thread {} start !!!", thread_id);
    }
    panic!("run loop")
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn bss_end();
    }
    (sbss as usize..bss_end as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
