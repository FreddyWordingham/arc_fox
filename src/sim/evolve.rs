//! Evolve functions.

use crate::{report, world::Universe};
use contracts::pre;
use log::info;
use ndarray::{Array1, Array3};
use physical_constants::BOLTZMANN_CONSTANT;
use std::f64::consts::PI;

/// Assumed temperature.
const TEMP: f64 = 293.15; // 40C.

/// Run a time evolution simulation.
#[pre(num_threads > 0)]
#[pre(end_time > 0.0)]
#[pre(dump_time > 0.0)]
#[pre(dump_time < end_time)]
pub fn run(
    num_threads: usize,
    end_time: f64,
    dump_time: f64,
    uni: &Universe,
) -> Array3<Array1<f64>> {
    info!("Running evolution simulation.");

    let mut concs = uni.grid().concs().map(|c| (*c).clone());
    let sources = uni.grid().sources().map(|s| (*s).clone());

    let diffusion_coeffs = diffusion_coeffs(uni);
    let max_coeff = find_max_coeff(&diffusion_coeffs).expect("No diffusion possible.");
    report!("Maximum diffusion coeff: {}", max_coeff);

    let mut curr_time = 0.0;
    let mut next_dump_time = 0.0;
    while curr_time < end_time {
        let dt = (end_time - curr_time) / 100.0;

        diffuse(&mut concs, &sources, &diffusion_coeffs, dt);

        curr_time += dt;

        if curr_time >= next_dump_time {
            println!("Dumping at t: {}", curr_time);
            next_dump_time += dump_time;
        }
    }

    concs
}

/// Initialise the diffusion coefficients.
fn diffusion_coeffs(uni: &Universe) -> Array3<Option<Array1<Option<f64>>>> {
    info!("Constructing diffusion coefficent map.");
    let mol_map = uni.mol_map();

    let mut coeffs = Vec::with_capacity(uni.grid().res().total());
    for cell in uni.grid().cells().iter() {
        let mat = cell.mat();

        if let Some(visc) = mat.visc() {
            let mut mol_coeffs: Array1<Option<f64>> = Array1::from_elem(mol_map.len(), None);

            for ((_id, mol), c) in mol_map.iter().zip(mol_coeffs.iter_mut()) {
                if let Some(rad) = mol.rad() {
                    *c = Some((BOLTZMANN_CONSTANT * TEMP) / (6.0 * PI * visc * rad));
                    report!((*c).unwrap());
                }
            }

            coeffs.push(Some(mol_coeffs));
        } else {
            coeffs.push(None);
        }
    }

    Array3::from_shape_vec(*uni.grid().res().arr(), coeffs)
        .expect("Unable to form data-cube of diffusion coefficients.")
}

/// Determine the maximum diffusion coefficient.
fn find_max_coeff(coeffs: &Array3<Option<Array1<Option<f64>>>>) -> Option<f64> {
    let mut max = None;
    for cs in coeffs.iter() {
        if let Some(cs) = cs {
            for c in cs.iter() {
                if let Some(c) = c {
                    if max.is_none() || max.unwrap() < *c {
                        max = Some(*c);
                    }
                }
            }
        }
    }

    max
}

/// Perform a diffusion step.
#[pre(dt > 0.0)]
fn diffuse(
    concs: &mut Array3<Array1<f64>>,
    sources: &Array3<Array1<f64>>,
    diffusion_coeffs: &Array3<Option<Array1<Option<f64>>>>,
    dt: f64,
) {
}
