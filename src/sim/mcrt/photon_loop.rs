//! MCRT photon-loop functions.

use crate::{
    sci::{
        math::{rng::distribution::henyey_greenstein, rt::Trace},
        phys::{Crossing, Photon},
    },
    sim::mcrt::{Hit, LightMap, Record, BUMP_DIST, MAX_LOOPS},
    util::{
        list::dimension::Cartesian::{X, Y, Z},
        progress::ParallelBar,
    },
    world::{dom::Cell, mat::Environment, parts::Light, Universe},
};
use contracts::pre;
use log::warn;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
};

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
                let mut shifted = false;
                let mut cell_rec = cell_and_record(&phot, universe, &mut lightmap);
                cell_rec.1.emissions += phot.weight();
                let mut env = cell_rec
                    .0
                    .mat_at_pos(phot.ray().pos())
                    .unwrap()
                    .optics()
                    .env(phot.wavelength());

                let mut num_loops = 0;
                loop {
                    num_loops += 1;
                    if num_loops >= MAX_LOOPS {
                        warn!(
                            "Photon prematurely killed as number of loops exceeded {}",
                            MAX_LOOPS
                        );
                    }

                    let scat_dist = -(rng.gen_range(0.0_f64, 1.0)).ln() / env.inter_coeff;
                    let cell_dist = cell_rec.0.boundary().dist(phot.ray()).unwrap();
                    let inter_dist = cell_rec.0.inter_dist(phot.ray());

                    match Hit::new(scat_dist, cell_dist, inter_dist) {
                        Hit::Scattering(dist) => {
                            cell_rec.1.dist_travelled += dist;
                            phot.travel(dist);

                            cell_rec.1.scatters += phot.weight();
                            phot.rotate(
                                henyey_greenstein(&mut rng, env.asym),
                                rng.gen_range(0.0, 2.0 * PI),
                            );

                            cell_rec.1.absorptions += env.albedo * phot.weight();
                            phot.multiply_weight(env.albedo);

                            if !shifted && rng.gen_range(0.0, 1.0) <= env.shift_prob {
                                cell_rec.1.shifts += phot.weight();
                                shifted = true;
                            }
                        }
                        Hit::Cell(dist) => {
                            let dist = dist + BUMP_DIST;
                            cell_rec.1.dist_travelled += dist;
                            phot.travel(dist);

                            if !universe.grid().dom().contains(phot.ray().pos()) {
                                break;
                            }

                            cell_rec = cell_and_record(&phot, universe, &mut lightmap);
                        }
                        Hit::Interface(_dist) => {
                            hit_interface(&mut rng, &mut phot, &mut cell_rec, &mut env);
                        }
                        Hit::InterfaceCell(_dist) => {
                            hit_interface(&mut rng, &mut phot, &mut cell_rec, &mut env);

                            if !universe.grid().dom().contains(phot.ray().pos()) {
                                break;
                            }

                            cell_rec = cell_and_record(&phot, universe, &mut lightmap);
                        }
                    }
                }
            }
            // === PHOTON LIFETIME ===
        }
    }

    lightmap
}

/// Perform an interface hit event.
fn hit_interface(
    rng: &mut ThreadRng,
    phot: &mut Photon,
    cell_rec: &mut (&Cell, &mut Record),
    env: &mut Environment,
) {
    let (dist, inside, norm, inter) = cell_rec.0.inter_dist_inside_norm_inter(phot.ray()).unwrap();

    let next_mat = if inside {
        inter.out_mat()
    } else {
        inter.in_mat()
    };
    let next_env = next_mat.optics().env(phot.wavelength());

    let n_curr = env.ref_index;
    let n_next = next_env.ref_index;

    let crossing = Crossing::new(phot.ray().dir(), &norm, n_curr, n_next);

    if rng.gen_range(0.0, 1.0) <= crossing.ref_prob() {
        let effective_dist = dist - BUMP_DIST;
        cell_rec.1.dist_travelled += effective_dist;
        phot.travel(effective_dist);
        phot.set_dir(*crossing.ref_dir());
    } else {
        let effective_dist = dist + BUMP_DIST;
        cell_rec.1.dist_travelled += effective_dist;
        phot.travel(effective_dist);
        phot.set_dir(crossing.trans_dir().unwrap());

        *env = next_env;
    }
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
