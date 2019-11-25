//! Evolve sub-module.

pub mod serial;
pub mod state;
pub mod statemap;

pub use self::state::*;
pub use self::statemap::*;

use crate::world::Universe;
use contracts::pre;
use log::info;
use ndarray::{Array1, Array3};
use physical_constants::BOLTZMANN_CONSTANT;
use std::f64::consts::PI;

/// Human body temperature.
const TEMPERATURE: f64 = 310.15;

/// Run an Evolution simulation.
#[pre(num_threads > 0)]
#[pre(sim_time > 0.0)]
#[pre(dump_time > 0.0)]
pub fn run(num_threads: usize, sim_time: f64, dump_time: f64, uni: &Universe) -> Statemap {
    info!("Running Evolution simulation.");

    let mut statemap = Statemap::new(uni.grid());
    let ds = diff_concs(&uni);
    let mut max_d = None;
    for ds in ds
        .iter()
        .filter(|ds| ds.is_some())
        .map(|ds| ds.as_ref().unwrap())
    {
        for d in ds.iter().filter(|d| d.is_some()) {
            if max_d.is_none() || d.unwrap() > max_d.unwrap() {
                max_d = *d;
            }
        }
    }
    info!("Max d: {}", max_d.unwrap());
    let cell_size = uni.grid().cell_size();
    let dx = cell_size.x.min(cell_size.y.min(cell_size.z));
    info!("Min dx: {}", dx);
    let min_diff_dt = dx.powi(2) / (4.0 * max_d.unwrap().powi(2));
    info!("Min dt: {}s", min_diff_dt);

    if num_threads == 1 {
        serial::run(sim_time, dump_time, &mut statemap);
    } else {
        unimplemented!("Coming soon!");
    }

    info!("Evolution complete.");

    statemap
}

/// Determine the diffusion coefficents for the given universe.
fn diff_concs(uni: &Universe) -> Array3<Option<Array1<Option<f64>>>> {
    let diff_concs = uni.grid().cells().map(|cell| {
        let mat = cell.mat();

        if let Some(visc) = mat.visc() {
            let coeffs: Vec<Option<f64>> = uni
                .mol_map()
                .iter()
                .map(|(_name, mol)| {
                    if let Some(rad) = mol.rad() {
                        let d = (BOLTZMANN_CONSTANT * TEMPERATURE) / (6.0 * PI * rad * visc);
                        Some(d)
                    } else {
                        None
                    }
                })
                .collect();

            return Some(Array1::from(coeffs));
        }

        return None;
    });

    diff_concs
}
