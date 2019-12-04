//! MCRT photon-loop functions.

use crate::{
    sci::{phys::Photon, rt::Trace},
    sim::mcrt::{LightMap, Record, MAX_LOOPS},
    util::{
        list::dimension::Cartesian::{X, Y, Z},
        progress::ParallelBar,
    },
    world::{dom::Cell, parts::Light, Universe},
};
use contracts::pre;
use log::warn;
use rand::thread_rng;
use std::sync::{Arc, Mutex};

/// Start a single-threaded photon loop.
#[pre(num_phot > 0)]
pub fn start(
    thread_id: usize,
    pb: Arc<Mutex<ParallelBar>>,
    num_phot: u64,
    light: &Light,
    universe: &Universe,
) -> LightMap {
    let shape: [usize; 3] = [
        universe.grid().cells().shape()[X as usize],
        universe.grid().cells().shape()[Y as usize],
        universe.grid().cells().shape()[Z as usize],
    ];
    let mut lightmap = LightMap::new(shape, universe.grid().cell_vol());

    loop {
        let start_end = { pb.lock().unwrap().inc(thread_id, 100) };
        if start_end.is_none() {
            break;
        }
        let (start, end) = start_end.unwrap();

        let mut rng = thread_rng();
        for _ in start..end {
            // === PHOTON LIFETIME ===
            {
                let mut phot = light.emit(&mut rng, num_phot);
                let mut _shifted = false;
                let mut cell_rec = cell_and_record(&phot, universe, &mut lightmap);
                cell_rec.1.emissions += phot.weight();
                let mut _env = cell_rec
                    .0
                    .mat_at_pos(phot.ray().pos())
                    .unwrap()
                    .env(phot.wavelength());

                for _ in 0..MAX_LOOPS {
                    // let scat_dist = -(rng.gen_range(0.0f64, 1.0)).ln() / env.inter_coeff();
                    let _cell_dist = cell_rec.0.boundary().dist(phot.ray()).unwrap();
                    let _inter_dist = cell_rec.0.inter_dist(phot.ray());
                    break;
                    // let scat_dist =
                }
                warn!(
                    "Photon prematurely killed as number of loops exceeded {}",
                    MAX_LOOPS
                );
            }
            // === PHOTON LIFETIME ===
        }
    }

    lightmap
}

/// Retrieve a reference for the cell corresponding record a photon is located within.
fn cell_and_record<'a>(
    phot: &Photon,
    uni: &'a Universe,
    lightmap: &'a mut LightMap,
) -> (&'a Cell<'a>, &'a mut Record) {
    let grid = uni.grid();
    let dom = grid.dom();
    let mins = dom.mins();
    let maxs = dom.maxs();
    let shape = grid.cells().shape();

    let id: Vec<usize> = phot
        .ray()
        .pos()
        .iter()
        .zip(mins.iter().zip(maxs.iter()))
        .zip(shape)
        .map(|((p, (min, max)), n)| index(*p, *min, *max, *n))
        .collect();
    let index = (id[0], id[1], id[2]);

    let cell = &uni.grid().cells()[index];
    let rec = &mut lightmap.recs[index];

    if !cell.boundary().contains(phot.ray().pos()) {
        panic!("Not inside that cell!"); // TODO: Remove
    }

    (cell, rec)
}

#[pre(x >= min)]
#[pre(x <= max)]
fn index(x: f64, min: f64, max: f64, res: usize) -> usize {
    (((x - min) / (max - min)) * res as f64) as usize
}
