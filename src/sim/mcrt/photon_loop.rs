//! MCRT photon-loop functions.

use crate::{
    report,
    sci::{
        math::{rng::distribution::henyey_greenstein, rt::Trace},
        phys::{Crossing, Photon},
    },
    sim::mcrt::{Hit, LightMap, Record, MAX_LOOPS},
    util::{
        list::dimension::Cartesian::{X, Y, Z},
        progress::ParallelBar,
    },
    world::{dom::Cell, mat::Environment, parts::Light, Universe},
};
use contracts::{post, pre};
use log::warn;
use nalgebra::{Point3, Unit};
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::{
    f64::{consts::PI, MIN_POSITIVE},
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
                //println!("A PHOTON IS BORN!~@");
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

                    let roulette = 0.1;
                    if phot.weight() < roulette {
                        if rng.gen_range(0.0_f64, 1.0) <= roulette {
                            phot.multiply_weight(1.0 / roulette);
                        } else {
                            break;
                        }
                    }

                    let scat_dist = -(rng.gen_range(0.0_f64, 1.0)).ln() / env.inter_coeff;
                    let cell_dist = cell_rec.0.boundary().dist(phot.ray()).unwrap();
                    let inter_dist = cell_rec.0.inter_dist(phot.ray());
                    match Hit::new(scat_dist, cell_dist, inter_dist, universe.bump_dist()) {
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

                            if !shifted && rng.gen_range(0.0, 1.0) <= 100.0 * env.shift_prob {
                                let m = env.shift_prob / 0.1;
                                phot.multiply_weight(0.01);
                                cell_rec.1.shifts += phot.weight();
                                shifted = true;
                            }
                            if shifted {
                                cell_rec.1.det_raman += peel_off(
                                    phot.clone(),
                                    env.clone(),
                                    &universe,
                                    &Point3::new(0.0129, 0.0, 0.0),
                                )
                                .unwrap_or(0.0);
                            }
                        }
                        Hit::Cell(dist) => {
                            let dist = dist + universe.bump_dist();
                            cell_rec.1.dist_travelled += dist;
                            phot.travel(dist);

                            if !universe.grid().dom().contains(phot.ray().pos()) {
                                if shifted {
                                    //let check = phot.ray().pos().y*phot.ray().pos().y + phot.ray().pos().z*phot.ray().pos().z;
                                    //if phot.ray().pos().x >= 0.0129 && check <= 0.000001{
                                    //    cell_rec.1.det_raman += phot.weight();
                                    //}
                                }
                                break;
                            }

                            cell_rec = cell_and_record(&phot, universe, &mut lightmap);
                        }
                        Hit::Interface(dist) => {
                            hit_interface(
                                &mut rng,
                                &mut phot,
                                &mut cell_rec,
                                &mut env,
                                dist,
                                universe.bump_dist(),
                            );

                            if !cell_rec.0.boundary().contains(phot.ray().pos()) {
                                // TODO: This should be able to be removed.
                                if !universe.grid().dom().contains(phot.ray().pos()) {
                                    if shifted {
                                        //let check = phot.ray().pos().y*phot.ray().pos().y + phot.ray().pos().z*phot.ray().pos().z;
                                        //if phot.ray().pos().x >= 0.0129 && check <= 0.000001{
                                        //    cell_rec.1.det_raman += phot.weight();
                                        //}
                                    }
                                    break;
                                }

                                warn!("It happened!");
                                cell_rec = cell_and_record(&phot, universe, &mut lightmap);
                            }
                        }
                        Hit::InterfaceCell(dist) => {
                            hit_interface(
                                &mut rng,
                                &mut phot,
                                &mut cell_rec,
                                &mut env,
                                dist,
                                universe.bump_dist(),
                            );

                            if !universe.grid().dom().contains(phot.ray().pos()) {
                                if shifted {
                                    //let check = phot.ray().pos().y*phot.ray().pos().y + phot.ray().pos().z*phot.ray().pos().z;
                                    //if phot.ray().pos().x >= 0.0129 && check <= 0.000001{
                                    //    cell_rec.1.det_raman += phot.weight();
                                    //}
                                }
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
#[pre(dist > 0.0)]
#[pre(bump_dist > 0.0)]
fn hit_interface(
    rng: &mut ThreadRng,
    phot: &mut Photon,
    cell_rec: &mut (&Cell, &mut Record),
    env: &mut Environment,
    dist: f64,
    bump_dist: f64,
) {
    let (_dist, inside, norm, inter) = cell_rec.0.inter_dist_inside_norm_inter(phot.ray()).unwrap();

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
        let effective_dist = (dist - bump_dist).max(MIN_POSITIVE);
        cell_rec.1.dist_travelled += effective_dist;
        phot.travel(effective_dist);
        phot.set_dir(*crossing.ref_dir());
    } else {
        let effective_dist = dist + bump_dist;
        cell_rec.1.dist_travelled += effective_dist;
        phot.travel(effective_dist);
        phot.set_dir(crossing.trans_dir().unwrap());

        *env = next_env;
    }
}

/// Retrieve a reference for the cell and corresponding record a photon is located within.
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
pub fn index(x: f64, min: f64, max: f64, res: usize) -> usize {
    (((x - min) / (max - min)) * res as f64) as usize
}

#[post(ret.is_none() || ret.unwrap() >= 0.0)]
pub fn peel_off(
    mut phot: Photon,
    mut env: Environment,
    uni: &Universe,
    pos: &Point3<f64>,
) -> Option<f64> {
    let g = env.asym;
    let g2 = g.powi(2);

    let dir = Unit::new_normalize(pos - phot.ray().pos());

    let cos_ang = phot.ray().dir().dot(&dir);
    let mut prob = phot.weight() * 0.5 * ((1.0 - g2) / (1.0 + g2 - (2.0 * g * cos_ang)).powf(1.5));
    //if prob < 0.01 {
    //    return None;
    //}

    phot.set_dir(dir);
    let mut cell = find_cell(&phot, uni);

    loop {
        if prob < 0.00001 {
            return None;
        }

        let cell_dist = cell.boundary().dist(phot.ray()).unwrap();
        let inter_dist = cell.inter_dist_inside_norm_inter(phot.ray());

        if let Some((dist, inside, _norm, inter)) = inter_dist {
            if dist < cell_dist {
                prob *= (-(dist + uni.bump_dist()) * env.inter_coeff).exp();
                phot.travel(dist + uni.bump_dist());

                if inside {
                    env = inter.in_mat().optics().env(phot.wavelength());
                } else {
                    env = inter.out_mat().optics().env(phot.wavelength());
                }
            } else {
                prob *= (-(cell_dist + uni.bump_dist()) * env.inter_coeff).exp();
                phot.travel(cell_dist + uni.bump_dist());
            }
        } else {
            prob *= (-(cell_dist + uni.bump_dist()) * env.inter_coeff).exp();
            phot.travel(cell_dist + uni.bump_dist());
        }

        if !uni.grid().dom().contains(phot.ray().pos()) {
            break;
        }

        cell = find_cell(&phot, uni);
    }
    //report!(prob);
    Some(prob)
}

/// Retrieve a reference for the cell a photon is located within.
pub fn find_cell<'a>(phot: &Photon, uni: &'a Universe) -> &'a Cell<'a> {
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

    if !cell.boundary().contains(phot.ray().pos()) {
        panic!("Not inside that cell!"); // TODO: Remove
    }

    cell
}
