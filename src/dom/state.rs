//! State structure.

use contracts::pre;
use ndarray::Array1;

/// State structure implementation.
/// Physical state of the cell.
#[derive(Debug)]
pub struct State {
    /// Molecule concentrations.
    concs: Array1<f64>,
    /// Molecule sources.
    sources: Array1<f64>,
}

impl State {
    /// Construct a new instance.
    #[pre(!concs.is_empty())]
    #[pre(concs.len() == sources.len())]
    pub fn new(concs: Array1<f64>, sources: Array1<f64>) -> Self {
        Self { concs, sources }
    }

    /// Reference the molecule concentrations.
    pub fn concs(&self) -> &Array1<f64> {
        &self.concs
    }

    /// Reference the molecule sources.
    pub fn sources(&self) -> &Array1<f64> {
        &self.sources
    }
}
