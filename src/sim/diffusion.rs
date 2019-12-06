//! Diffusion simulation sub-module.

pub mod serial;

pub use self::serial::*;

use crate::world::Universe;
use contracts::pre;
use nalgebra::Vector3;
use ndarray::Array3;
use ndarray_stats::QuantileExt;

/// Run a time diffusion simulation.
#[pre(num_threads > 0)]
#[pre(time > 0.0)]
pub fn run(num_threads: usize, time: f64, universe: &mut Universe) {
    let mut concs = Vec::with_capacity(universe.species().len());
    let mut coeffs = Vec::with_capacity(universe.species().len());

    for (index, _species) in universe.species().iter().enumerate() {
        concs.push(
            universe
                .grid()
                .cells()
                .map(|cell| cell.state().concs()[index]),
        );
        coeffs.push(
            universe
                .grid()
                .cells()
                .map(|cell| cell.state().diff_coeffs()[index]),
        );
    }

    let max_coeffs: Vec<_> = coeffs
        .iter()
        .map(|ds| ds.map(|d| d.unwrap_or(0.0)))
        .map(|ds| *ds.max().unwrap())
        .collect();
    let widths = universe.grid().cell_widths();
    let dx = widths.min();
    let max_dts = max_coeffs.iter().map(|d| dx.powi(2) / (4.0 * d));

    for ((mut concs, coeffs), max_dt) in concs.iter_mut().zip(coeffs).zip(max_dts) {
        let dt = max_dt / 10.0;
        let n = ((time / dt) as u64).max(1);
        let dt = time / n as f64;

        for i in 0..n {
            println!("{} of {}", i, n);
            diffuse(&mut concs, &coeffs, dt, &widths);
        }
    }

    // for (concs, state) in concs
    for (index, concs) in concs.iter().enumerate() {
        for (cell, conc) in universe.grid_mut().cells_mut().iter_mut().zip(concs) {
            cell.state_mut().concs_mut()[index] = *conc;
        }
    }
}

/// Perform a diffusion step on one species.
pub fn diffuse(
    concs: &mut Array3<f64>,
    coeffs: &Array3<Option<f64>>,
    dt: f64,
    widths: &Vector3<f64>,
) {
    let old = concs.clone();
    let shape = old.shape();

    for xi in 0..shape[0] {
        let prev_x = if xi == 0 { shape[0] - 1 } else { xi - 1 };
        let next_x = if xi == (shape[0] - 1) { 0 } else { xi + 1 };
        for yi in 0..shape[1] {
            let prev_y = if yi == 0 { shape[1] - 1 } else { yi - 1 };
            let next_y = if yi == (shape[1] - 1) { 0 } else { yi + 1 };
            for zi in 0..shape[2] {
                let prev_z = if zi == 0 { shape[2] - 1 } else { zi - 1 };
                let next_z = if zi == (shape[2] - 1) { 0 } else { zi + 1 };

                let index = [xi, yi, zi];

                if let Some(coeff) = coeffs[index] {
                    concs[index] += coeff
                        * (((old[[prev_x, yi, zi]] - (2.0 * old[index]) + old[[next_x, yi, zi]])
                            / widths.x.powi(2))
                            + ((old[[xi, prev_y, zi]] - (2.0 * old[index])
                                + old[[xi, next_y, zi]])
                                / widths.x.powi(2))
                            + ((old[[xi, yi, prev_z]] - (2.0 * old[index])
                                + old[[xi, yi, next_z]])
                                / widths.x.powi(2)))
                        * dt;
                }
            }
        }
    }
}
