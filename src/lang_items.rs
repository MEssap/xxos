use crate::println;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let line = _info.location().expect("Err").line();
    let file = _info.location().expect("err").file();
    let message = _info.message().expect("err");
    println!("{}:{} {}", file, line, message);
    loop {}
}
