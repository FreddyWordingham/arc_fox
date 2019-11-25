//! Domain module.

pub mod cell;
pub mod grid;

pub use self::cell::*;
pub use self::grid::*;

/// Detection increase fraction.
const SIGMA: f64 = 0.01;
