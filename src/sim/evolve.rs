//! Evolution simulation sub-module.

use crate::world::Universe;
use contracts::pre;

/// Run a time evolution simulation.
#[pre(num_threads > 0)]
#[pre(time > 0.0)]
pub fn run(num_threads: usize, time: f64, universe: &mut Universe) {
    let pre_concs = universe.grid().cells().map(|cell| cell.state().concs());
    let mults = universe
        .grid()
        .cells()
        .map(|cell| cell.mat().reaction_multiplier());
    let mut post_concs = universe.grid().cells().map(|cell| cell.state().concs());

    for ((pre_cs, _post_cs), mult) in pre_concs
        .iter()
        .zip(post_concs.iter_mut())
        .zip(mults.iter())
    {
        let mut t = 0.0;
        while t < time {
            let mut rates = Vec::with_capacity(universe.reactions().len());
            for react in universe.reactions() {
                rates.push(react.rate().res(pre_cs) * mult);
            }

            // Determine dt.
            let dt = 0.01;
            t += dt;

            for (react, _rate) in universe.reactions().iter().zip(rates.iter()) {
                for (_index, _stoi) in react.reactants() {
                    // cell.state_mut().concs_mut()[*index] -= *stoi as f64 * rate * dt;
                    // post_cs[*index] -= *stoi as f64 * rate * dt;
                }
            }
        }
    }
}
