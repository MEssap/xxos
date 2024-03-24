pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

//pub fn run_next_app() -> ! {
//    let mut app_manager = APP_MANAGER.exclusive_access();
//    let current_app = app_manager.get_current_app();
//    unsafe {
//        app_manager.load_app(current_app);
//    }
//    app_manager.move_to_next_app();
//    drop(app_manager);
//    // before this we have to drop local variables related to resources manually
//    // and release the resources
//    extern "C" {
//        fn __restore(cx_addr: usize);
//    }
//    unsafe {
//        __restore(KERNEL_STACK.push_context(TrapContext::app_init_context(
//            APP_BASE_ADDRESS,
//            USER_STACK.get_sp(),
//        )) as *const _ as usize);
//    }
//    panic!("Unreachable in batch::run_current_app!");
//}
