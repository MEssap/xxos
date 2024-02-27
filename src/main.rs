#![no_main]
#![no_std]

use alloc::vec::Vec;
use core::arch::global_asm;
use core::sync::atomic::{AtomicBool, Ordering};
use xxos::console::Log;
use xxos::mm;
use xxos::mm::vm::kvm::kvmmake;
use xxos::opensbi::thread_start;
use xxos::println;
//use xxos::mm::pagetable::pgtb_test;
//use xxos::proc::context_test;
//use xxos::riscv::riscv_test;

static STARTED: AtomicBool = AtomicBool::new(false);
extern crate alloc;
global_asm!(include_str!("entry.s"));

#[no_mangle]
fn main() {
    thread_start();

    // 仅由id为0的线程执行初始化操作
    let thread_id = xxos::opensbi::r_tp();
    if thread_id == 0 {
        clear_bss();

        xxos_log::init_log(&Log, xxos_log::Level::WARN); // 初始化log

        // 初始化内存
        mm::pm::heap_init();
        let kvm = kvmmake();

        // test
        //pgtb_test();
        //context_test();
        //riscv_test();
        let mut vec: Vec<u8> = alloc::vec::Vec::with_capacity(0x5000);
        vec.push(1);
        println!("vec {:?}", vec);
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
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
