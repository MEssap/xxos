#![no_main]
#![no_std]

use core::arch::global_asm;
use core::sync::atomic::{AtomicBool, Ordering};
use xxos::console::Log;
use xxos::riscv::registers::r_tp;
use xxos::trap::trampoline::usertrapret;
use xxos::{mm, proc, utils};
use xxos::{println, trap};
static STARTED: AtomicBool = AtomicBool::new(false);
extern crate alloc;
global_asm!(include_str!("entry.s"));

#[no_mangle]
fn main() {
    //thread_start();

    // 仅由id为0的线程执行初始化操作
    let thread_id = r_tp();
    if thread_id < 10 {
        //清理bss段
        utils::clear_bss();
        // 初始化系统log
        xxos_log::init_log(&Log, xxos_log::Level::WARN);
        // 初始化trap
        trap::kerneltrap::kernel_trap_init();
        //rap::ecall::user_env_call_init();
        //trap::clock::clock_init();
        // 初始化内存
        mm::pm::heap_init();
        // 初始化虚拟内存
        mm::vm::kvm_init();
        proc::process::test_initcode();
        // test
        //trap_test();

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
        // 每个CPU都使用KVM的页表
        mm::vm::kvm_init();
        println!("Thread {} start !!!", thread_id);
    }
    usertrapret();
    panic!("run loop")
}
