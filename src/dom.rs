//! Domain module.

pub mod cell;
pub mod grid;
pub mod region;
pub mod state;

pub use self::cell::*;
pub use self::grid::*;
pub use self::region::*;
pub use self::state::*;

/// Detection increase fraction.
const SIGMA: f64 = 0.01;
