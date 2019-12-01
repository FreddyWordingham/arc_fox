//! Progress bar utility sub-module.

pub mod parallel_bar;
pub mod serial_bar;

pub use self::parallel_bar::*;
pub use self::serial_bar::*;
