//! Monte-carlo radiative transfer.

use crate::{
    data::Archive,
    util::progress::bar,
    world::{Light, Universe},
};
use contracts::pre;
use indicatif::ProgressBar;
use rand::thread_rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run a MCRT simulation.
#[pre(num_threads > 0)]
pub fn run(num_threads: usize, total_phot: u64, light: &Light, uni: &Universe) -> Archive {
    let num_phots = Arc::new(Mutex::new(vec![0; num_threads]));
    let bar = Arc::new(bar("photon loop", total_phot));

    if num_threads == 1 {
        let thread_id = 0;
        return run_thread(thread_id, total_phot, num_phots, bar, light, uni);
    }

    let thread_ids: Vec<usize> = (0..num_threads).collect();
    let mut archives: Vec<Archive> = thread_ids
        .par_iter()
        .map(|id| run_thread(*id, total_phot, num_phots.clone(), bar.clone(), light, uni))
        .collect();

    let mut archive = archives.pop().unwrap();
    for a in archives.iter() {
        archive += a;
    }

    bar.finish_with_message("Photon loop complete.");

    archive
}

/// Run a mcrt simulation behaving as a single thread.
fn run_thread(
    thread_id: usize,
    total_phot: u64,
    mut num_phots: Arc<Mutex<Vec<u64>>>,
    mut bar: Arc<ProgressBar>,
    light: &Light,
    uni: &Universe,
) -> Archive {
    let archive = Archive::new(uni.grid().layout().clone());

    let mut rng = thread_rng();

    while iterate(&mut bar, thread_id, total_phot, &mut num_phots) {
        let _phot = light.emit(&mut rng, total_phot);
    }

    archive
}

/// Iterate the progress one increment if possible.
fn iterate(
    bar: &mut Arc<ProgressBar>,
    thread_id: usize,
    total_phot: u64,
    num_phots: &mut Arc<Mutex<Vec<u64>>>,
) -> bool {
    let mut num_phots = num_phots
        .lock()
        .expect("Could not lock the number of photons!");

    let sum_phot: u64 = num_phots.iter().sum();
    if total_phot < sum_phot {
        bar.inc(1);
        num_phots[thread_id] += 1;
        return true;
    }

    false
}
