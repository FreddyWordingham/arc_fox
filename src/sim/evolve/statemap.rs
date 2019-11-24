//! Statemap structure.

use crate::{dom::Grid, sim::evolve::State, world::MolMap};
use ndarray::Array3;

/// Statemap structure implementation.
/// Record Statemap.
#[derive(Debug)]
pub struct Statemap {
    /// Cell volume [m^2].
    cell_vol: f64,
    /// State array.
    pub states: Array3<State>,
}

impl Statemap {
    /// Construct a new instance.
    pub fn new(grid: &Grid) -> Self {
        Self {
            cell_vol: grid.dom().vol() / grid.res().total() as f64,
            states: grid.cells().map(|cell| cell.state().clone()),
        }
    }

    /// Get the concentration map of a given molecule.
    fn mol_conc(&self, id: usize) -> Array3<f64> {
        self.states.map(|s| s.concs()[id])
    }

    /// Get the source map of a given molecule.
    fn mol_source(&self, id: usize) -> Array3<f64> {
        self.states.map(|s| s.sources()[id])
    }

    /// Get a map of all molecule's spatial concentrations.
    pub fn mol_concs<'a>(&self, mol_map: &'a MolMap) -> Vec<(&'a str, Array3<f64>)> {
        let mut mcs = Vec::with_capacity(self.states[(0, 0, 0)].concs().len());

        for (id, (name, _mol)) in mol_map.iter().enumerate() {
            mcs.push((name.as_str(), self.mol_conc(id)));
        }

        mcs
    }

    /// Get a map of all molecule's spatial sources.
    pub fn source_concs<'a>(&self, mol_map: &'a MolMap) -> Vec<(&'a str, Array3<f64>)> {
        let mut mcs = Vec::with_capacity(self.states[(0, 0, 0)].concs().len());

        for (id, (name, _mol)) in mol_map.iter().enumerate() {
            mcs.push((name.as_str(), self.mol_source(id)));
        }

        mcs
    }
}
