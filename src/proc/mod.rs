use self::context::Context;
use xxos_log::info;

pub(crate) mod context;
pub(crate) mod def;
pub(crate) mod process;

pub fn context_test() {
    let origin_context = Context::new();
    let mut context = Context::new();

    info!("context test start");

    info!("get context:\n{:#x?}", context);
    unsafe { context.store_context() };
    unsafe { origin_context.store_context() };
    info!("after stored:\n{:#x?}", context);

    context.test(0xdeadbeef);
    info!("now I set context's s1 = 0xdeadbeef\n{:#x?}", context);
    info!("and then load it to register.");
    unsafe { context.load_context() };

    context.test(0);
    info!("now I set context's s1 = 0\n{:#x?}", context);
    unsafe { context.store_context() };
    info!("and then store from register.");
    info!("get context:\n{:#x?}", context);

    info!("context test end");
    unsafe { origin_context.load_context() };
}
