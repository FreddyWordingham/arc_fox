//! Run in parallel functions.

use super::serial;
use crate::{data::Archive, opt::Light, world::Universe};
use contracts::pre;
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run a MCRT simulation in parallel.
#[pre(num_threads > 1)]
pub fn run(
    num_threads: usize,
    total_phot: u64,
    num_phots: Arc<Mutex<Vec<u64>>>,
    bar: Arc<ProgressBar>,
    light: &Light,
    uni: &Universe,
) -> Archive {
    let thread_ids: Vec<usize> = (0..num_threads).collect();
    let mut archives: Vec<Archive> = thread_ids
        .par_iter()
        .map(|id| serial::run(*id, total_phot, num_phots.clone(), bar.clone(), light, uni))
        .collect();

    let mut archive = archives.pop().unwrap();
    for a in archives.iter() {
        archive += a;
    }

    archive
}
