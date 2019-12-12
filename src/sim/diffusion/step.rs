//! Diffusion stepping function.

use crate::util::{
    list::dimension::Cartesian::{X, Y, Z},
    progress::ParallelBar,
};
use contracts::pre;
use nalgebra::Vector3;
use ndarray::Array3;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Perform a diffusion step for a single species.
#[pre(num_threads > 0)]
#[pre(!concs.is_empty())]
#[pre(!coeffs.is_empty())]
#[pre(cell_size.iter().all(|x | *x > 0.0))]
#[pre(time > 0.0)]
pub fn step_species(
    num_threads: usize,
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

    let max_dt = cell_size.min().powi(2) / (4.0 * max_coeff); // TODO: Consider smaller.

    let steps = (time / max_dt).ceil() as u64;
    let dt = time / steps as f64;

    for qs in 0..steps {
        println!("QS: {} / {}", qs, steps);
        quick_step(num_threads, concs, coeffs, cell_size, dt);
    }
}

/// Perform a single step.
#[pre(num_threads > 0)]
#[pre(!concs.is_empty())]
#[pre(!coeffs.is_empty())]
#[pre(dt > 0.0)]
pub fn quick_step(
    num_threads: usize,
    concs: &mut Array3<f64>,
    coeffs: &Array3<Option<f64>>,
    cell_size: &Vector3<f64>,
    dt: f64,
) {
    let pb = Arc::new(Mutex::new(ParallelBar::new(
        "Diffusing",
        concs.len() as u64,
        num_threads,
    )));
    let thread_ids: Vec<usize> = (0..num_threads).collect();
    let results: Vec<_> = thread_ids
        .par_iter()
        .map(|id| thread_stp(*id, Arc::clone(&pb), concs, coeffs, cell_size))
        .collect();
    pb.lock()
        .unwrap()
        .finish_with_message("Species diffusion step done.");

    let mut rate: Array3<f64> = Array3::zeros(concs.dim());
    for result in results {
        for (a, b) in result.iter().zip(rate.iter_mut()) {
            if let Some(c) = a {
                *b = *c;
            }
        }
    }

    *concs = rate * dt;
}

/// Diffuse on a single thread.
#[pre(!concs.is_empty())]
#[pre(!coeffs.is_empty())]
#[pre(concs.shape() == coeffs.shape())]
pub fn thread_stp(
    thread_id: usize,
    pb: Arc<Mutex<ParallelBar>>,
    concs: &Array3<f64>,
    coeffs: &Array3<Option<f64>>,
    cell_size: &Vector3<f64>,
) -> Array3<Option<f64>> {
    let num_cells = [concs.shape()[0], concs.shape()[1], concs.shape()[2]];
    let mut result = Array3::from_elem(num_cells, None);

    loop {
        let start_end = { pb.lock().unwrap().inc(thread_id, 100) };
        if start_end.is_none() {
            break;
        }

        let (start, end) = start_end.unwrap();

        for n in start..end {
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

                result[[xi, yi, zi]] = Some(
                    coeff
                        * (((concs[[prev_x, yi, zi]] - (2.0 * concs[[xi, yi, zi]])
                            + concs[[next_x, yi, zi]])
                            / cell_size[X as usize].powi(2))
                            + ((concs[[xi, prev_y, zi]] - (2.0 * concs[[xi, yi, zi]])
                                + concs[[xi, next_y, zi]])
                                / cell_size[Y as usize].powi(2))
                            + ((concs[[xi, yi, prev_z]] - (2.0 * concs[[xi, yi, zi]])
                                + concs[[xi, yi, next_z]])
                                / cell_size[Z as usize].powi(2))),
                );
            }
        }
    }

    result
}
