//! State structure.

use crate::world::{map::index_of_key, MolMap};
use contracts::pre;
use ndarray::Array1;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// State structure implementation.
/// Physical state of the cell.
#[derive(Debug, Clone)]
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

    /// Construct a new empty state.
    pub fn new_empty(len: usize) -> Self {
        Self {
            concs: Array1::zeros(len),
            sources: Array1::zeros(len),
        }
    }

    /// Build an instance from a proto-state.
    #[pre(!mol_map.is_empty())]
    pub fn build(mol_map: &MolMap, proto_state: &ProtoState) -> Self {
        let mut concs = Array1::zeros(mol_map.len());
        let mut sources = Array1::zeros(mol_map.len());

        for (id, conc) in proto_state.concs().iter() {
            concs[index_of_key(mol_map, id)] = *conc;
        }

        for (id, source) in proto_state.sources().iter() {
            sources[index_of_key(mol_map, id)] = *source;
        }

        Self::new(concs, sources)
    }

    /// Reference the molecule concentrations.
    pub fn concs(&self) -> &Array1<f64> {
        &self.concs
    }

    /// Mutably reference the molecule concentrations.
    pub fn mut_concs(&mut self) -> &mut Array1<f64> {
        &mut self.concs
    }

    /// Reference the molecule sources.
    pub fn sources(&self) -> &Array1<f64> {
        &self.sources
    }
}

/// Proto-State structure implementation.
/// Stores information required to build a state.
#[derive(Debug, Deserialize, Serialize)]
pub struct ProtoState {
    /// Molecule concentration-map.
    concs: HashMap<String, f64>,
    /// Molecule source-map.
    sources: HashMap<String, f64>,
}

impl ProtoState {
    /// Construct a new instance.
    #[pre(!concs.is_empty())]
    #[pre(concs.len() == sources.len())]
    pub fn new(concs: HashMap<String, f64>, sources: HashMap<String, f64>) -> Self {
        Self { concs, sources }
    }

    /// Reference the molecule concentration-map.
    pub fn concs(&self) -> &HashMap<String, f64> {
        &self.concs
    }

    /// Reference the molecule source-map.
    pub fn sources(&self) -> &HashMap<String, f64> {
        &self.sources
    }
}
