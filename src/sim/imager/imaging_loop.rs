//! MC imaging functions.

use crate::{
    sci::{
        math::{rng::distribution::henyey_greenstein, rt::Trace},
        phys::{Crossing, Photon},
    },
    sim::{
        imager::Camera,
        mcrt::{photon_loop::index, Hit, MAX_LOOPS},
    },
    util::progress::ParallelBar,
    world::{dom::Cell, mat::Environment, parts::Light, Universe},
};
use contracts::{post, pre};
use log::warn;
use nalgebra::Unit;
use ndarray::Array2;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::{
    f64::{consts::PI, MIN_POSITIVE},
    sync::{Arc, Mutex},
};

/// Start a single-threaded photon loop.
#[pre(num_phot > 0)]
#[pre(power > 0)]
pub fn start(
    thread_id: usize,
    pb: Arc<Mutex<ParallelBar>>,
    num_phot: u64,
    light: &Light,
    universe: &Universe,
    cam: &Camera,
    power: u32,
) -> Array2<f64> {
    let res = 2_usize.pow(power);
    let mut arr: Array2<f64> = Array2::zeros([res, res]);

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
                let mut cell = find_cell(&phot, universe);
                let mut env = cell
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
                    let cell_dist = cell.boundary().dist(phot.ray()).unwrap();
                    let inter_dist = cell.inter_dist(phot.ray());

                    match Hit::new(scat_dist, cell_dist, inter_dist, universe.bump_dist()) {
                        Hit::Scattering(dist) => {
                            phot.travel(dist);
                            phot.rotate(
                                henyey_greenstein(&mut rng, env.asym),
                                rng.gen_range(0.0, 2.0 * PI),
                            );
                            phot.multiply_weight(env.albedo);

                            if !shifted && rng.gen_range(0.0, 1.0) <= env.shift_prob {
                                shifted = true;
                            }

                            cam.observe(
                                &mut arr,
                                phot.ray().pos(),
                                peel_off(phot.clone(), env.clone(), &universe, &cam),
                            );
                        }
                        Hit::Cell(dist) => {
                            let dist = dist + universe.bump_dist();
                            phot.travel(dist);

                            if !universe.grid().dom().contains(phot.ray().pos()) {
                                break;
                            }

                            cell = find_cell(&phot, universe);
                        }
                        Hit::Interface(dist) => {
                            hit_interface(
                                &mut rng,
                                &mut phot,
                                cell,
                                &mut env,
                                dist,
                                universe.bump_dist(),
                            );
                        }
                        Hit::InterfaceCell(dist) => {
                            hit_interface(
                                &mut rng,
                                &mut phot,
                                cell,
                                &mut env,
                                dist,
                                universe.bump_dist(),
                            );

                            if !universe.grid().dom().contains(phot.ray().pos()) {
                                break;
                            }

                            cell = find_cell(&phot, universe);
                        }
                    }
                }
            }
            // === PHOTON LIFETIME ===
        }
    }

    arr
}

/// Perform an interface hit event.
#[pre(bump_dist > 0.0)]
pub fn hit_interface(
    rng: &mut ThreadRng,
    phot: &mut Photon,
    cell: &Cell,
    env: &mut Environment,
    dist: f64,
    bump_dist: f64,
) {
    let (_dist, inside, norm, inter) = cell.inter_dist_inside_norm_inter(phot.ray()).unwrap();

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
        phot.travel(effective_dist);
        phot.set_dir(*crossing.ref_dir());
    } else {
        let effective_dist = dist + bump_dist;
        phot.travel(effective_dist);
        phot.set_dir(crossing.trans_dir().unwrap());

        *env = next_env;
    }
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

#[post(ret > 0.0)]
pub fn peel_off(mut phot: Photon, mut env: Environment, uni: &Universe, cam: &Camera) -> f64 {
    let g = env.asym;
    let g2 = g.powi(2);

    let dir = Unit::new_normalize(cam.pos() - phot.ray().pos());

    let cos_ang = phot.ray().dir().dot(&dir);
    let mut prob = 0.5 * ((1.0 - g2) / (1.0 + g2 - (2.0 * g * cos_ang)).powf(1.5));
    if prob < 0.1 {
        return 0.0;
    }

    phot.set_dir(dir);
    let mut cell = find_cell(&phot, uni);

    while prob >= 0.1 {
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

    prob
}
