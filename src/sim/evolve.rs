//! Evolve sub-module.

pub mod parallel;
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
use std::{f64::consts::PI, path::Path};

/// Human body temperature.
const TEMPERATURE: f64 = 310.15;

/// Run an Evolution simulation.
#[pre(out_dir.is_dir())]
#[pre(num_threads > 0)]
#[pre(sim_time > 0.0)]
#[pre(dump_time > 0.0)]
pub fn run(
    out_dir: &Path,
    num_threads: usize,
    sim_time: f64,
    dump_time: f64,
    uni: &Universe,
) -> Statemap {
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
    let max_dt = dx.powi(2) / (4.0 * max_d.unwrap());
    let max_dt = max_dt / 10.0;
    info!("Max dt: {}s", max_dt);

    let multipliers = uni
        .grid()
        .cells()
        .map(|cell| cell.mat().reaction_multiplier());

    // if num_threads == 1 {
    serial::run(
        out_dir,
        sim_time,
        dump_time,
        max_dt,
        uni.grid().res(),
        &cell_size,
        &ds,
        uni.mol_map(),
        uni.react_map(),
        &mut statemap,
        &multipliers,
    );
    // } else {
    //     parallel::run(
    //         num_threads,
    //         out_dir,
    //         sim_time,
    //         dump_time,
    //         max_dt,
    //         uni.grid().res(),
    //         &cell_size,
    //         &ds,
    //         uni.mol_map(),
    //         uni.react_map(),
    //         &mut statemap,
    //     );
    // }

    info!("Evolution complete.");

    statemap
}

/// Determine the diffusion coefficients for the given universe.
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
