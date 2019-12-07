//! Monte-Carlo imaging sub-module.

pub mod camera;
pub mod imaging_loop;

pub use self::camera::*;

use crate::{
    util::progress::ParallelBar,
    world::{parts::Light, Universe},
};
use contracts::pre;
use ndarray::Array2;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run an monte-carlo imaging simulation.
#[pre(num_threads > 0)]
#[pre(num_phot > 0)]
#[pre(power > 0)]
pub fn run(
    num_threads: usize,
    num_phot: u64,
    light: &Light,
    universe: &Universe,
    cam: &Camera,
    power: u32,
) -> Array2<f64> {
    let pb = Arc::new(Mutex::new(ParallelBar::new(
        "Running Imager",
        num_phot,
        num_threads,
    )));

    let thread_ids: Vec<usize> = (0..num_threads).collect();
    let mut images: Vec<Array2<f64>> = thread_ids
        .par_iter()
        .map(|id| imaging_loop::start(*id, Arc::clone(&pb), num_phot, light, universe, cam, power))
        .collect();
    pb.lock().unwrap().finish_with_message("Imager complete.");

    let mut image = images.pop().unwrap();
    for img in images {
        image += &img;
    }

    image
}
