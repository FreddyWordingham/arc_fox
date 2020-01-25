//! State structure.

use crate::access;

/// Chemical species state set.
pub struct State {
    /// Current concentration.
    conc: f64,
    /// Current source/sink rate.
    source: f64,
}

impl State {
    access!(conc, f64);
    access!(source, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(conc: f64, source: f64) -> Self {
        Self { conc, source }
    }
}
