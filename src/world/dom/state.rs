//! State structure.

use contracts::pre;
use ndarray::Array1;

/// State structure implementation.
/// Physical state of the cell.
#[derive(Debug)]
pub struct State {
    /// Species concentrations.
    concs: Array1<f64>,
    /// Species sources.
    sources: Array1<f64>,
}

impl State {
    /// Construct a new instance.
    #[pre(concs.len() == sources.len())]
    pub fn new(concs: Array1<f64>, sources: Array1<f64>) -> Self {
        Self { concs, sources }
    }
}
