//! Run in serial functions.

use super::sample;
use crate::{data::Archive, opt::Light, world::Universe};
use indicatif::ProgressBar;
use rand::thread_rng;
use std::sync::{Arc, Mutex};

/// Run a mcrt simulation in serial.
pub fn run(
    thread_id: usize,
    total_phot: u64,
    mut num_phots: Arc<Mutex<Vec<u64>>>,
    mut bar: Arc<ProgressBar>,
    light: &Light,
    uni: &Universe,
) -> Archive {
    let res = uni.grid().res().clone();
    let mut archive = Archive::new(res);

    let mut rng = thread_rng();

    while iterate(&mut bar, thread_id, total_phot, &mut num_phots) {
        sample::photon_life(&mut archive, &mut rng, total_phot, light, uni);
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
    let mut num_phots = num_phots.lock().unwrap();

    let sum_phot: u64 = num_phots.iter().sum();
    if sum_phot < total_phot {
        bar.inc(1);
        num_phots[thread_id] += 1;
        return true;
    }

    false
}
