//! State structure.

use crate::{
    sci::chem::Species,
    world::{dom::StateBuilder, parts::index_of_name},
};
use contracts::pre;
use ndarray::Array1;

/// State structure implementation.
/// Physical state of the cell.
#[derive(Debug, Clone)]
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

    /// Build a new instance.
    pub fn build(builder: StateBuilder, species: &[Species]) -> Self {
        let mut init_concs = Array1::zeros(species.len());
        let mut init_sources = Array1::zeros(species.len());

        if let Some(concs) = builder.concs {
            for (name, conc) in concs {
                init_concs[index_of_name(species, &name)] = conc;
            }
        }

        if let Some(sources) = builder.sources {
            for (name, source) in sources {
                init_sources[index_of_name(species, &name)] = source;
            }
        }

        Self::new(init_concs, init_sources)
    }

    /// Reference the species concentrations.
    pub fn concs(&self) -> &Array1<f64> {
        &self.concs
    }

    /// Reference the species sources.
    pub fn sources(&self) -> &Array1<f64> {
        &self.sources
    }
}
