//! MCRT functions.

use crate::{
    data::{Archive, Record},
    dom::Cell,
    opt::{Light, Photon},
    util::progress::bar,
    world::Universe,
};
use contracts::{post, pre};
use indicatif::ProgressBar;
use log::info;
use rand::{rngs::ThreadRng, thread_rng};
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
        run_photon(&mut archive, &mut rng, total_phot, light, uni);
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

/// Simulate the life of a single photon.
fn run_photon(
    mut archive: &mut Archive,
    mut rng: &mut ThreadRng,
    total_phot: u64,
    light: &Light,
    uni: &Universe,
) {
    let mut phot = light.emit(&mut rng, total_phot);
    let mut cell_rec = cell_and_record(&phot, uni, &mut archive);
    cell_rec.1.increase_emissions(phot.weight());
    let mut env = cell_rec
        .0
        .mat_at_pos(&phot.ray().pos(), uni.grid().dom(), uni.inter_map())
        .env(phot.wavelength());
    let mut shifted = false;
}

/// Retrieve a reference for the cell corresponding record a photon is located within.
#[post(ret.0.aabb().contains(phot.ray().pos()))]
fn cell_and_record<'a>(
    phot: &Photon,
    uni: &Universe,
    archive: &'a mut Archive,
) -> (&'a Cell<'a>, &'a mut Record) {
    let index = uni
        .grid()
        .dom()
        .find_index(phot.ray().pos(), uni.grid().res());

    (
        &uni.grid().cells()[*index.arr()],
        &mut archive.recs()[*index.arr()],
    )
}
