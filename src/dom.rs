//! Domain module.

pub mod cell;
pub mod grid;
pub mod region;

pub use self::cell::*;
pub use self::grid::*;
pub use self::region::*;

/// Detection increase fraction.
const SIGMA: f64 = 0.01;
