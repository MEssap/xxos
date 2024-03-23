pub mod input;
pub mod output;

// the macro about print
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::output::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::output::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

pub struct Log;
impl xxos_log::WriteLog for Log {
    fn print(&self, log_content: core::fmt::Arguments) {
        println!("{}", log_content);
    }
}
