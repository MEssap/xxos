use super::error_trace::ErrorTrace;
pub type Result<'a,T> = core::result::Result<T,ErrorTrace<'a>>;

