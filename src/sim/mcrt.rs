//! MCRT sub-module.

pub mod parallel;
pub mod sample;
pub mod serial;

use crate::{data::Archive, opt::Light, util::Monitor, world::Universe};
use contracts::pre;
use log::info;
use std::sync::{Arc, Mutex};

/// Run a MCRT simulation.
#[pre(num_threads > 0)]
pub fn run(num_threads: usize, total_phot: u64, light: &Light, uni: &Universe) -> Archive {
    info!("Running MCRT simulation.");

    let monitor = Arc::new(Mutex::new(Monitor::new(
        "Photon loop",
        total_phot,
        num_threads,
    )));

    let archive = if num_threads == 1 {
        serial::run(0, total_phot, Arc::clone(&monitor), light, uni)
    } else {
        parallel::run(num_threads, total_phot, Arc::clone(&monitor), light, uni)
    };

    monitor
        .lock()
        .unwrap()
        .finish_with_message("Photon loop complete.");

    archive
}
