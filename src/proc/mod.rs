use self::context::Context;
use xxos_log::info;

pub(crate) mod context;
pub(crate) mod def;
pub(crate) mod process;

pub fn context_test() {
    let mut context = Context::new();
    info!("get context:\n{:#?}", context);
    unsafe { context.store_context() };
    info!("after stored:\n{:#?}", context);

    context.test(0xdeadbeef);
    info!("now I set context's sp = 0xdeadbeef\n{:#?}", context);
    info!("and then load it to register.");
    unsafe { context.load_context() };

    context.test(0);
    info!("now I set context's sp = 0\n{:#?}", context);
    unsafe { context.store_context() };
    info!("and then store from register.");
    info!("get context:\n{:#?}", context);
}
