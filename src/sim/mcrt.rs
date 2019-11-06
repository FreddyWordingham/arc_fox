//! Monte-carlo radiative transfer.

use crate::{
    data::{Archive, Record},
    dom::Cell,
    index::bin,
    phys::opt::Photon,
    rng::henyey_greenstein,
    rt::Traceable,
    util::progress::bar,
    world::{Entity, Light, Universe},
};
use contracts::pre;
use indicatif::ProgressBar;
use log::info;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
};

/// Run a MCRT simulation.
#[pre(num_threads > 0)]
pub fn run(num_threads: usize, total_phot: u64, light: &Light, uni: &Universe) -> Archive {
    let num_phots = Arc::new(Mutex::new(vec![0; num_threads]));

    if num_threads == 1 {
        info!("Running as single thread.");
        let bar = Arc::new(bar("photon loop", total_phot));
        return run_thread(0, total_phot, num_phots, bar, light, uni);
    }

    info!("Running multi-thread ({}).", num_threads);
    let bar = Arc::new(bar("photon loop", total_phot));
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
    let mut archive = Archive::new(uni.grid().layout().clone());

    let mut rng = thread_rng();

    while iterate(&mut bar, thread_id, total_phot, &mut num_phots) {
        let mut phot = light.emit(&mut rng, total_phot);
        let mut cell_rec = cell_and_record(&phot, uni, &mut archive);
        let mat = cell_rec.0.mat_at_pos(&phot.ray().pos);
        let env = mat.env(phot.wavelength());

        cell_rec.1.increase_emissions(&phot);

        loop {
            let inter_dist = -(rng.gen_range(0.0f64, 1.0)).ln() / env.inter_coeff;
            let cell_dist = cell_rec.0.aabb().dist(phot.ray()).unwrap();
            let ent_info = cell_rec.0.ent_dist_inside(phot.ray());

            match HitEvent::new(inter_dist, cell_dist, ent_info) {
                HitEvent::Scattering { dist } => {
                    cell_rec.1.increase_scatters(&phot);

                    phot.travel(dist);
                    phot.rotate(
                        henyey_greenstein(&mut rng, env.asym),
                        rng.gen_range(0.0, 2.0 * PI),
                    );

                    phot.multiply_weight(env.albedo);
                }
                HitEvent::Boundary { dist } => {
                    phot.travel(dist + 0.00_000_1);

                    if !uni.grid().aabb().contains(&phot.ray().pos) {
                        break;
                    }

                    cell_rec = cell_and_record(&phot, uni, &mut archive);
                }
                HitEvent::Entity { dist } => {}
            }
        }
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

/// Retrieve a reference for the cell corresponding record a photon is located within.
fn cell_and_record<'a>(
    phot: &Photon,
    uni: &'a Universe,
    archive: &'a mut Archive,
) -> (&'a Cell<'a>, &'a mut Record) {
    let index = bin::point3(&phot.ray().pos, uni.grid().aabb(), uni.grid().layout());

    (&uni.grid().cells()[index], &mut archive.recs[index])
}

/// Hit event types.
enum HitEvent {
    /// Scattering event.
    Scattering {
        /// Distance to the scattering event.
        dist: f64,
    },
    /// Cell boundary collision.
    Boundary {
        /// Distance to the cell surface.
        dist: f64,
    },
    /// Entity surface collision.
    Entity {
        /// Distance to the entity surface.
        dist: f64,
    },
}

impl HitEvent {
    #[pre(dist > 0.0)]
    pub fn new_scattering(dist: f64) -> Self {
        Self::Scattering { dist }
    }

    #[pre(dist > 0.0)]
    pub fn new_boundary(dist: f64) -> Self {
        Self::Boundary { dist }
    }

    #[pre(dist > 0.0)]
    pub fn new_entity(dist: f64) -> Self {
        Self::Entity { dist }
    }

    #[pre(inter_dist > 0.0)]
    #[pre(cell_dist > 0.0)]
    pub fn new<'a>(
        inter_dist: f64,
        cell_dist: f64,
        ent_info: Option<(&'a Entity<'a>, f64, bool)>,
    ) -> Self {
        if cell_dist <= inter_dist {
            if let Some((_ent, ent_dist, _inside)) = ent_info {
                if ent_dist < cell_dist {
                    return Self::new_entity(ent_dist);
                }
            }

            return Self::new_boundary(cell_dist);
        }

        if let Some((_ent, ent_dist, _inside)) = ent_info {
            if ent_dist < inter_dist {
                return Self::new_entity(ent_dist);
            }
        }

        Self::new_scattering(inter_dist)
    }
}
