#![allow(unused)]

use crate::opensbi::Opensbi;
use core::fmt::{self, Write};
use xx_mutex_lock::Mutex;

pub static PT: Mutex<Writer> = Mutex::new(Writer);

pub struct Writer;

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            Opensbi::console_putchar(c as usize);
        }
        Ok(())
    }
}




pub fn print(args: fmt::Arguments) {
    PT.lock().write_fmt(args).unwrap();
}
