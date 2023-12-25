use core::panic::Location;
pub struct ErrorTrace<'a> {
    pub message: &'a str,
    file: &'a str,
    line: u32
}

impl<'a> ErrorTrace<'a> {
    #[track_caller]
    pub fn new(message: &str) -> ErrorTrace {
        let location = Location::caller();
        ErrorTrace {
            message,
            file:location.file(),
            line: location.line()
        }
    }
}

impl<'a> core::fmt::Display for ErrorTrace<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f,"Error cause {} in {}:{}",self.message,self.file,self.line)
    }
}

//an example 
// impl From<std::io::Error> for ErrorTrace {
//     #[track_caller]
//     fn from(value: std::io::Error) -> Self {
//         let message = value.to_string();
//         let location = std::panic::Location::caller();
//         ErrorTrace::from_other_error(&message, location.file(), location.line())
//     }
// }