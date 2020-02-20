//! State implementation.

use crate::access;
use ndarray::Array1;

/// Local species state.
pub struct State {
    /// Species concentrations.
    concs: Array1<f64>,
    /// Species sources.
    sources: Array1<f64>,
}

impl State {
    access!(concs, concs_mut, Array1<f64>);
    access!(sources, Array1<f64>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(concs: Array1<f64>, sources: Array1<f64>) -> Self {
        Self { concs, sources }
    }

    /// Construct a new empty instance.
    #[inline]
    #[must_use]
    pub fn empty(len: usize) -> Self {
        Self::new(Array1::zeros(len), Array1::zeros(len))
    }
}
