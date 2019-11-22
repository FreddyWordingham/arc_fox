//! Run in parallel functions.

use super::{serial, Lightmap};
use crate::{opt::Light, util::Monitor, world::Universe};
use contracts::pre;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run a MCRT simulation in parallel.
#[pre(num_threads > 1)]
pub fn run(
    num_threads: usize,
    total_phot: u64,
    monitor: Arc<Mutex<Monitor>>,
    light: &Light,
    uni: &Universe,
) -> Lightmap {
    let thread_ids: Vec<usize> = (0..num_threads).collect();
    let mut lightmaps: Vec<Lightmap> = thread_ids
        .par_iter()
        .map(|id| serial::run(*id, total_phot, Arc::clone(&monitor), light, uni))
        .collect();

    let mut lightmap = lightmaps.pop().unwrap();
    for a in lightmaps.iter() {
        lightmap += a;
    }

    lightmap
}
