use super::error_trace::ErrorTrace;
pub type Result<T> = core::result::Result<T, ErrorTrace>;
