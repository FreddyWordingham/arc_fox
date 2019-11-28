//! Progress bar utility sub-module.

pub mod parallel;
pub mod serial;

pub use self::parallel::*;
pub use self::serial::*;
