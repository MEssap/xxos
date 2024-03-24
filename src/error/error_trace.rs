use alloc::string::{String, ToString};
use core::panic::Location;

pub struct ErrorTrace {
    pub message: String,
    file: String,
    line: u32,
}

impl ErrorTrace {
    #[track_caller]
    pub fn new(message: &str) -> ErrorTrace {
        let location = Location::caller();
        ErrorTrace {
            message: message.to_string(),
            file: location.file().to_string(),
            line: location.line(),
        }
    }
    pub fn from_other_error(message: &str, file: &str, line: u32) -> ErrorTrace {
        ErrorTrace {
            message: message.to_string(),
            file: file.to_string(),
            line,
        }
    }
}

impl core::fmt::Display for ErrorTrace {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Error cause [{}] in {}:{}",
            self.message, self.file, self.line
        )
    }
}
