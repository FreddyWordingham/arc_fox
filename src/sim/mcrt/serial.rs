//! Run in serial functions.

use super::{sample, Lightmap};
use crate::{opt::Light, util::Monitor, world::Universe};
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
) -> Lightmap {
    let res = uni.grid().res().clone();
    let mut lightmap = Lightmap::new(uni.grid().dom().vol() / res.total() as f64, res);

    let mut rng = thread_rng();

    while monitor.lock().unwrap().inc(thread_id).is_some() {
        sample::photon_life(&mut lightmap, &mut rng, total_phot, light, uni);
    }

    lightmap
}
