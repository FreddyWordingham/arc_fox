//! MCRT sub-module.

pub mod parallel;
pub mod sample;
pub mod serial;

use crate::{data::Archive, opt::Light, util::progress::bar, world::Universe};
use contracts::pre;
use log::info;
use std::sync::{Arc, Mutex};

/// Run a MCRT simulation.
#[pre(num_threads > 0)]
pub fn run(num_threads: usize, total_phot: u64, light: &Light, uni: &Universe) -> Archive {
    info!("Running MCRT simulation.");

    let num_phots = Arc::new(Mutex::new(vec![0; num_threads]));
    let bar = Arc::new(bar("photon loop", total_phot));

    let archive = if num_threads == 1 {
        info!("Running as single thread.");
        serial::run(0, total_phot, num_phots.clone(), bar.clone(), light, uni)
    } else {
        info!("Running multi-threaded ({}).", num_threads);
        parallel::run(
            num_threads,
            total_phot,
            num_phots.clone(),
            bar.clone(),
            light,
            uni,
        )
    };

    bar.finish_with_message("Photon loop complete.");

    info!("Thread reports:");
    for (thread_id, num_phot) in num_phots.lock().unwrap().iter().enumerate() {
        println!(
            "\tThread {}: {} phots ({:.2}%)",
            thread_id,
            num_phot,
            *num_phot as f64 / total_phot as f64 * 100.0
        );
    }

    archive
}
