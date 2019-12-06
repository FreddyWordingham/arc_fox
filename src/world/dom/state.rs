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
    /// Species diffusion coefficients.
    diff_coeffs: Array1<Option<f64>>,
}

impl State {
    /// Construct a new instance.
    #[pre(concs.len() == sources.len())]
    #[pre(diff_coeffs.iter().all(|d| d.is_none() || d.unwrap() > 0.0))]
    pub fn new(concs: Array1<f64>, sources: Array1<f64>, diff_coeffs: Array1<Option<f64>>) -> Self {
        Self {
            concs,
            sources,
            diff_coeffs,
        }
    }

    /// Build a new instance.
    #[pre(species.len() == diff_coeffs.len())]
    pub fn build(
        builder: StateBuilder,
        diff_coeffs: Array1<Option<f64>>,
        species: &[Species],
    ) -> Self {
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

        Self::new(init_concs, init_sources, diff_coeffs)
    }

    /// Reference the species concentrations.
    pub fn concs(&self) -> &Array1<f64> {
        &self.concs
    }

    /// Reference the species concentrations mutably.
    pub fn concs_mut(&mut self) -> &mut Array1<f64> {
        &mut self.concs
    }

    /// Reference the species sources.
    pub fn sources(&self) -> &Array1<f64> {
        &self.sources
    }

    /// Reference the species diffusion coefficients.
    pub fn diff_coeffs(&self) -> &Array1<Option<f64>> {
        &self.diff_coeffs
    }
}
