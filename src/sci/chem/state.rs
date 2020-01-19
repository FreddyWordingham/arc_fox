//! State structure.

use crate::{
    access,
    ord::Set,
    sci::chem::{Reaction, Species, StateBuilder},
};
use ndarray::Array1;

/// Chemical species state set.
pub struct State {
    /// Species concentration.
    concs: Array1<f64>,
    /// Source / sink terms.
    sources: Array1<f64>,
}

impl State {
    access!(concs, concs_mut, Array1<f64>);
    access!(sources, Array1<f64>);

    /// Build a new instance.
    #[inline]
    #[must_use]
    pub fn build(builder: StateBuilder, species: &[Species]) -> Self {
        let mut concs: Array1<f64> = Array1::zeros(species.len());
        let mut sources: Array1<f64> = Array1::zeros(species.len());

        if let Some(builder_concs) = builder.concs() {
            for (name, conc) in builder_concs {
                let index = species.index_of(&name).expect("Unknown species name.");
                *concs.get_mut(index).expect("Invalid species index.") = *conc;
            }
        }

        if let Some(builder_sources) = builder.sources() {
            for (name, source) in builder_sources {
                let index = species.index_of(&name).expect("Unknown species name.");
                *sources.get_mut(index).expect("Invalid species index.") = *source;
            }
        }

        Self { concs, sources }
    }

    /// Calculate the current rate of change for the state.
    #[inline]
    #[must_use]
    pub fn rate_of_change(&self, reactions: &[Reaction]) -> Array1<f64> {
        let mut rates = self.sources.clone();

        for reaction in reactions {
            let rate = reaction.rate().calc(&self.concs);

            for (index, coeff) in reaction.reactants() {
                *rates.get_mut(*index).expect("Invalid species index.") += rate * coeff;
            }
            for (index, coeff) in reaction.products() {
                *rates.get_mut(*index).expect("Invalid species index.") -= rate * coeff;
            }
        }

        rates
    }
}
