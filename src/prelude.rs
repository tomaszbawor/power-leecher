pub use crate::error::Error;

#[allow(dead_code)]
pub type Result<T> = core::result::Result<T, Error>;

// Generic wrapper for any type
#[allow(dead_code)]
pub struct Wrapper<T>(pub T);
