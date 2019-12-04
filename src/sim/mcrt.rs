//! Monte-Carlo radiative transfer simulation sub-module.

pub mod hit;
pub mod light_map;
pub mod photon_loop;
pub mod record;

pub use self::hit::*;
pub use self::light_map::*;
pub use self::record::*;

use crate::{
    util::progress::ParallelBar,
    world::{parts::Light, Universe},
};
use contracts::pre;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run a monte-carlo radiative transfer simulation.
#[pre(num_threads > 0)]
#[pre(num_phot > 0)]
pub fn run(num_threads: usize, num_phot: u64, light: &Light, universe: &Universe) -> LightMap {
    let pb = Arc::new(Mutex::new(ParallelBar::new(
        "Running MCRT",
        num_phot,
        num_threads,
    )));

    let thread_ids: Vec<usize> = (0..num_threads).collect();
    let mut lightmaps: Vec<LightMap> = thread_ids
        .par_iter()
        .map(|id| photon_loop::start(*id, light, universe))
        .collect();
    pb.lock().unwrap().finish_with_message("MCRT complete.");

    let mut lightmap = lightmaps.pop().unwrap();
    for lm in lightmaps {
        lightmap += &lm;
    }

    lightmap
}
