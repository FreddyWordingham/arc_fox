//! Diffusion stepping function.

use crate::util::{
    list::dimension::Cartesian::{X, Y, Z},
    progress::SerialBar,
};
use contracts::pre;
use nalgebra::Vector3;
use ndarray::Array3;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Perform a diffusion step for a single species.
#[pre(!concs.is_empty())]
#[pre(!coeffs.is_empty())]
#[pre(cell_size.iter().all(|x | *x > 0.0))]
#[pre(time > 0.0)]
pub fn step_species(
    concs: &mut Array3<f64>,
    coeffs: &Array3<Option<f64>>,
    cell_size: &Vector3<f64>,
    time: f64,
) {
    let max_coeff = coeffs
        .iter()
        .filter_map(Option::as_ref)
        .max_by(|a, b| a.partial_cmp(b).expect("Invalid diffusion coefficient."))
        .unwrap();

    let max_dt = cell_size.min().powi(2) / (4.0 * max_coeff);
    let dt = max_dt / 10.0;

    let steps = (time / dt).ceil() as u64;
    let dt = time / steps as f64;

    if steps > 1000 {
        let mut pb = SerialBar::new("Diffusing", steps);
        for _ in 0..steps {
            pb.inc();
            quick_step(concs, coeffs, cell_size, dt);
        }
    } else {
        for _ in 0..steps {
            quick_step(concs, coeffs, cell_size, dt);
        }
    }
}

/// Perform a single step.
#[pre(!concs.is_empty())]
#[pre(!coeffs.is_empty())]
#[pre(dt > 0.0)]
pub fn quick_step(
    concs: &mut Array3<f64>,
    coeffs: &Array3<Option<f64>>,
    cell_size: &Vector3<f64>,
    dt: f64,
) {
    let num_cells = [concs.shape()[0], concs.shape()[1], concs.shape()[2]];
    let total_cells = concs.len();
    let mut rate = Arc::new(Mutex::new(Array3::zeros(num_cells)));

    (0..total_cells).into_par_iter().for_each(|n| {
        let n = n as usize;
        let zi = n % num_cells[X as usize];
        let yi = ((n - zi) / num_cells[X as usize]) % num_cells[Y as usize];
        let xi = (n - zi - (yi * num_cells[X as usize]))
            / (num_cells[X as usize] * num_cells[Y as usize]);

        if let Some(coeff) = coeffs[[xi, yi, zi]] {
            let prev_x = if xi == 0 {
                num_cells[X as usize] - 1
            } else {
                xi - 1
            };
            let next_x = if xi == (num_cells[X as usize] - 1) {
                0
            } else {
                xi + 1
            };
            let prev_y = if yi == 0 {
                num_cells[Y as usize] - 1
            } else {
                yi - 1
            };
            let next_y = if yi == (num_cells[Y as usize] - 1) {
                0
            } else {
                yi + 1
            };
            let prev_z = if zi == 0 {
                num_cells[Z as usize] - 1
            } else {
                zi - 1
            };
            let next_z = if zi == (num_cells[Z as usize] - 1) {
                0
            } else {
                zi + 1
            };

            rate.lock().unwrap()[[xi, yi, zi]] = coeff
                * (((concs[[prev_x, yi, zi]] - (2.0 * concs[[xi, yi, zi]])
                    + concs[[next_x, yi, zi]])
                    / cell_size[X as usize].powi(2))
                    + ((concs[[xi, prev_y, zi]] - (2.0 * concs[[xi, yi, zi]])
                        + concs[[xi, next_y, zi]])
                        / cell_size[Y as usize].powi(2))
                    + ((concs[[xi, yi, prev_z]] - (2.0 * concs[[xi, yi, zi]])
                        + concs[[xi, yi, next_z]])
                        / cell_size[Z as usize].powi(2)));
        }
    });

    let rate = Arc::try_unwrap(rate).unwrap().into_inner().unwrap();

    *concs += &(rate * dt);
}
