//! Mesh implementation.

use crate::{
    chem::Species,
    dom::{Name, Set},
    uni::State as UniState,
};
use attr::json;
use ndarray::Array1;
use std::collections::BTreeMap;

/// State initialisation structure.
#[json]
pub struct State {
    /// Species initial concentrations.
    concs: BTreeMap<Name, f64>,
    /// Species sources.
    sources: BTreeMap<Name, f64>,
}

impl State {
    /// Build a state.
    #[inline]
    #[must_use]
    pub fn build(&self, specs: &Set<Species>) -> UniState {
        let num_spec = specs.map().len();
        let mut concs = Array1::zeros(num_spec);
        let mut sources = Array1::zeros(num_spec);

        for (conc, name) in concs.iter_mut().zip(specs.map().keys()) {
            if self.concs.contains_key(name) {
                *conc = *self.concs.get(name).expect("Missing species name.");
            }
        }

        for (source, name) in sources.iter_mut().zip(specs.map().keys()) {
            if self.sources.contains_key(name) {
                *source = *self.sources.get(name).expect("Missing species name.");
            }
        }

        UniState::new(concs, sources)
    }
}
