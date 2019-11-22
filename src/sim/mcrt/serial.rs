//! Run in serial functions.

use super::sample;
use crate::{data::Archive, opt::Light, util::Monitor, world::Universe};
use contracts::pre;
use rand::thread_rng;
use std::sync::{Arc, Mutex};

/// Run a mcrt simulation in serial.
#[pre(total_phot > 0)]
pub fn run(
    thread_id: usize,
    total_phot: u64,
    monitor: Arc<Mutex<Monitor>>,
    light: &Light,
    uni: &Universe,
) -> Archive {
    let res = uni.grid().res().clone();
    let mut archive = Archive::new(res);

    let mut rng = thread_rng();

    while monitor.lock().unwrap().inc(thread_id) {
        sample::photon_life(&mut archive, &mut rng, total_phot, light, uni);
    }

    archive
}
