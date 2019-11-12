//! Monte-carlo radiative transfer.

use crate::{
    data::Archive,
    util::progress::bar,
    world::{Light, Universe},
};
use contracts::pre;
use indicatif::ProgressBar;
use log::info;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Distance to bump over boundaries to prevent getting stuck.
const BUMP_DIST: f64 = 1.0e-9;

/// Run a MCRT simulation.
#[pre(num_threads > 0)]
pub fn run(num_threads: usize, total_phot: u64, light: &Light, uni: &Universe) -> Archive {
    let num_phots = Arc::new(Mutex::new(vec![0; num_threads]));
    let bar = Arc::new(bar("photon loop", total_phot));

    if num_threads == 1 {
        info!("Running as single thread.");
        return run_thread(0, total_phot, num_phots, bar, light, uni);
    }

    info!("Running multi-thread ({}).", num_threads);
    let thread_ids: Vec<usize> = (0..num_threads).collect();
    let mut archives: Vec<Archive> = thread_ids
        .par_iter()
        .map(|id| run_thread(*id, total_phot, num_phots.clone(), bar.clone(), light, uni))
        .collect();
    bar.finish_with_message("Photon loop complete.");

    info!("Thread reports:");
    for (thread_id, num_phot) in num_phots.lock().unwrap().iter().enumerate() {
        println!(
            "\tThread {}: {} phots ({}%)",
            thread_id,
            num_phot,
            *num_phot as f64 / total_phot as f64 * 100.0
        );
    }

    info!("Stacking archives...");
    let mut archive = archives.pop().unwrap();
    for a in archives.iter() {
        archive += a;
    }

    archive
}

/// Run a mcrt simulation behaving as a single thread.
fn run_thread(
    _thread_id: usize,
    _total_phot: u64,
    mut _num_phots: Arc<Mutex<Vec<u64>>>,
    mut _bar: Arc<ProgressBar>,
    _light: &Light,
    uni: &Universe,
) -> Archive {
    let res = uni.grid().res().clone();
    let archive = Archive::new(res);

    archive
}
