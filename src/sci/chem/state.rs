//! State structure.

use crate::{
    ord::Set,
    sci::chem::{Reaction, Species, StateBuilder},
};
use ndarray::Array1;

/// Chemical species state set.
pub struct State {
    /// Species concentration.
    pub concs: Array1<f64>,
    /// Source / sink terms.
    pub sources: Array1<f64>,
}

impl State {
    /// Build a new instance.
    #[inline]
    pub fn build(proto: StateBuilder, species: &[Species]) -> Self {
        let mut concs = Array1::zeros(species.len());
        let mut sources = Array1::zeros(species.len());

        for (name, conc, source) in proto.init {
            let index = species.index_of(&name).expect("Unknown species name.");
            *concs.get_mut(index).expect("Invalid species index.") = conc;
            *sources.get_mut(index).expect("Invalid species index.") = source;
        }

        Self { concs, sources }
    }

    /// Evolve forward using the source terms only.
    #[inline]
    pub fn add_source(&mut self, dt: f64) {
        self.concs += &(&self.sources * dt);
    }

    /// Evolve forward using the competitive reactions.
    #[inline]
    pub fn evolve(&mut self, dt: f64, reactions: &[Reaction]) {
        let mut deltas = Array1::<f64>::zeros(self.concs.len());
        for reaction in reactions {
            let rate = reaction.rate.calc(&self.concs);

            for (index, coeff) in &reaction.reactants {
                *deltas.get_mut(*index).expect("Invalid species index.") += rate * coeff;
            }
            for (index, coeff) in &reaction.products {
                *deltas.get_mut(*index).expect("Invalid species index.") -= rate * coeff;
            }
        }

        for conc in self.concs.iter_mut() {
            if *conc <= 0.0 {
                *conc = 0.0;
            }
        }

        self.concs += &(&deltas * dt);
    }
}
