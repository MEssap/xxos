#![no_main]
#![no_std]

use alloc::vec::Vec;
use core::arch::global_asm;
use core::sync::atomic::{AtomicBool, Ordering};
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

    // 仅由id为0的线程执行初始化操作
    let thread_id = xxos::opensbi::r_tp();
    if thread_id == 0 {
        //清理bss段
        clear_bss();
        // 初始化系统log
        xxos_log::init_log(&Log, xxos_log::Level::WARN); // 初始化log
                                                         // 初始化内存
        mm::pm::heap_init();
        // 初始化虚拟内存
        mm::vm::kvm_init();

        // test
        //context_test();
        //riscv_test();
        println!("Thread {} start !!!", thread_id);
        STARTED.store(true, Ordering::SeqCst);
    } else {
        loop {
            if STARTED.load(Ordering::SeqCst) {
                break;
            }
        }
        mm::vm::kvm_init();
        println!("Thread {} start !!!", thread_id);
    }
    panic!("run loop")
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
