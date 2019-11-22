//! Sampling functions.

use super::{Lightmap, Record};
use crate::{
    dom::Cell,
    opt::{Light, Photon},
    rng::sample::henyey_greenstein,
    rt::{Gate, Hit, Trace},
    world::Universe,
};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

/// Distance to move past boundaries.
pub const BUMP_DIST: f64 = 1.0e-9; // [m].

/// Simulate the life of a single photon.
pub fn photon_life(
    mut lightmap: &mut Lightmap,
    mut rng: &mut ThreadRng,
    total_phot: u64,
    light: &Light,
    uni: &Universe,
) {
    let mut phot = light.emit(&mut rng, total_phot);
    let mut cell_rec = cell_and_record(&phot, &uni, &mut lightmap);
    cell_rec.1.increase_emissions(phot.weight());
    let mut env = cell_rec
        .0
        .mat_at_pos(&phot.ray().pos(), uni.grid().dom(), uni.inter_map())
        .env(phot.wavelength());
    let mut shifted = false;

    let mut loops = 0;
    loop {
        loops += 1;
        if loops > 1000 {
            break;
        }
        let scat_dist = -(rng.gen_range(0.0f64, 1.0)).ln() / env.inter_coeff();
        let cell_dist = cell_rec.0.aabb().dist(phot.ray()).unwrap();
        let inter_dist = cell_rec.0.inter_dist(phot.ray());

        match Hit::new(scat_dist, cell_dist, inter_dist) {
            Hit::Scattering(dist) => {
                phot.travel(dist);
                cell_rec.1.increase_dist_travelled(dist);

                phot.rotate(
                    henyey_greenstein(&mut rng, env.asym()),
                    rng.gen_range(0.0, 2.0 * PI),
                );
                cell_rec.1.increase_scatters(phot.weight());

                cell_rec
                    .1
                    .increase_absorptions(env.albedo() * phot.weight());
                phot.multiply_weight(env.albedo());

                if !shifted && rng.gen_range(0.0, 1.0) <= env.shift_prob() {
                    shifted = true;
                    cell_rec.1.increase_shifts(phot.weight());
                }
            }
            Hit::Cell(dist) => {
                phot.travel(dist + BUMP_DIST);
                cell_rec.1.increase_dist_travelled(dist + BUMP_DIST);

                if !uni.grid().dom().contains(phot.ray().pos()) {
                    break;
                }

                cell_rec = cell_and_record(&phot, &uni, &mut lightmap);
            }
            Hit::Interface(_dist) => {
                let (dist, inside, norm, inter) =
                    cell_rec.0.inter_dist_inside_norm_inter(phot.ray()).unwrap();

                let next_mat = if inside {
                    inter.out_mat()
                } else {
                    inter.in_mat()
                };
                let next_env = next_mat.env(phot.wavelength());

                let n_curr = env.ref_index();
                let n_next = next_env.ref_index();

                let gate = Gate::new(&phot.ray().dir(), &norm, n_curr, n_next);

                if rng.gen_range(0.0, 1.0) <= gate.ref_prob() {
                    phot.travel(dist - BUMP_DIST);
                    cell_rec.1.increase_dist_travelled(dist - BUMP_DIST);
                    phot.set_dir(*gate.ref_dir());
                } else {
                    phot.travel(dist + BUMP_DIST);
                    cell_rec.1.increase_dist_travelled(dist + BUMP_DIST);
                    phot.set_dir(gate.trans_dir().unwrap());

                    env = next_env;
                }
            }
            Hit::InterfaceCell(_dist) => {
                let (dist, inside, norm, inter) =
                    cell_rec.0.inter_dist_inside_norm_inter(phot.ray()).unwrap();

                let next_mat = if inside {
                    inter.out_mat()
                } else {
                    inter.in_mat()
                };
                let next_env = next_mat.env(phot.wavelength());

                let n_curr = env.ref_index();
                let n_next = next_env.ref_index();

                let gate = Gate::new(&phot.ray().dir(), &norm, n_curr, n_next);

                if rng.gen_range(0.0, 1.0) <= gate.ref_prob() {
                    phot.travel(dist - BUMP_DIST);
                    cell_rec.1.increase_dist_travelled(dist - BUMP_DIST);
                    phot.set_dir(*gate.ref_dir());
                } else {
                    phot.travel(dist + BUMP_DIST);
                    cell_rec.1.increase_dist_travelled(dist + BUMP_DIST);
                    phot.set_dir(gate.trans_dir().unwrap());

                    env = next_env;
                }

                if !uni.grid().dom().contains(&phot.ray().pos()) {
                    break;
                }

                cell_rec = cell_and_record(&phot, uni, &mut lightmap);
            }
        }
    }
}

/// Retrieve a reference for the cell corresponding record a photon is located within.
fn cell_and_record<'a>(
    phot: &Photon,
    uni: &'a Universe,
    lightmap: &'a mut Lightmap,
) -> (&'a Cell<'a>, &'a mut Record) {
    let index = uni
        .grid()
        .dom()
        .find_index(phot.ray().pos(), uni.grid().res())
        .arr()
        .clone();

    let cell = &uni.grid().cells()[index];
    let rec = &mut lightmap.recs[index];

    if !cell.aabb().contains(&phot.ray().pos()) {
        panic!("Not inside that cell!");
    }

    (cell, rec)
}
