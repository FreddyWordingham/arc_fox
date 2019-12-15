//! Diffusion simulation sub-module.

pub mod step;

use crate::world::Universe;
use contracts::pre;

/// Run a time diffusion simulation.
#[pre(time > 0.0)]
pub fn run(universe: &mut Universe, time: f64) {
    let cell_size = universe.grid().cells()[[0, 0, 0]].boundary().widths();

    for index in 0..universe.species().len() {
        let mut concs = universe
            .grid()
            .cells()
            .map(|cell| cell.state().concs()[index]);
        let coeffs = universe
            .grid()
            .cells()
            .map(|cell| cell.state().diff_coeffs()[index]);
        let sources = universe
            .grid()
            .cells()
            .map(|cell| cell.state().sources()[index]);

        concs += &(sources * time);

        step::step_species(&mut concs, &coeffs, &cell_size, time);

        for (cell, conc) in universe.grid_mut().cells_mut().iter_mut().zip(concs.iter()) {
            cell.state_mut().concs_mut()[index] = *conc;
        }
    }
}
