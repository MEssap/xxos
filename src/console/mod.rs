mod def;
pub mod stdin;
pub mod stdout;

// the macro about print
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::stdout::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::stdout::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
