//! Evolution simulation sub-module.

use crate::{
    sci::chem::Reaction,
    world::{dom::Cell, Universe},
};
use contracts::pre;

/// Run a time evolution simulation.
#[pre(num_threads > 0)]
#[pre(time > 0.0)]
pub fn run(num_threads: usize, time: f64, universe: &mut Universe) {
    let mut mults = universe
        .grid()
        .cells()
        .map(|cell| cell.mat().reaction_multiplier());

    // let reactions = universe.reactions();
    // universe
    // .grid_mut()
    // .cells_mut()
    // .into_par_iter()
    // .for_each(|cell| evolve_cell(cell, reactions, time));

    for (cell, mult) in universe.grid_mut().cells_mut().zip(mults.iter()) {
        // evolve_cell(cell, reactions, time);
        let state = cell.state_mut();
        for reaction in universe.reactions() {}
    }
}

/// Evolve a single cell.
#[pre(time > 0.0)]
pub fn evolve_cell(cell: &mut Cell, _reactions: &[Reaction], time: f64) {
    let _state = cell.state_mut();
}
