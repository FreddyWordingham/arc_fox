//! Monte-Carlo radiative transfer simulation sub-module.

pub mod hit;
pub mod light_map;
pub mod record;

pub use self::{hit::*, light_map::*, record::*};

use crate::{
    access,
    dom::{Cell, Name, Regular, Set},
    geom::Trace,
    math::henyey_greenstein,
    phys::{Crossing, Environment, Photon},
    uni::{Material, Verse},
    util::bar,
};
use log::warn;
use nalgebra::Point3;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::f64::{consts::PI, MIN_POSITIVE};

/// Maximum number of loops a photon will make before being culled prematurely.
const MAX_LOOPS: u64 = 10_000;

/// Weight below which to perform roulette each photon loop.
const ROULETTE: f64 = 0.1;

/// Generate a lightmap for a given setup.
#[inline]
#[must_use]
#[ignore(clippy::too_many_lines)]
pub fn run(name: &Name, num_phot: u64, verse: &Verse, grid: &Regular) -> LightMap {
    let bump_dist = grid.bump_dist();

    let pb = bar("Photon loop", num_phot);
    let mut rng = thread_rng();

    let light = &verse.lights().map().get(name).expect("Invalid light name.");
    let mut light_map = LightMap::new(grid.res(), grid.cell_vol());
    for _ in 0..num_phot {
        pb.inc(1);

        let mut phot = light.emit(&mut rng, num_phot, verse.meshes());
        let mut shifted = false;

        let mut cell_rec = cell_and_record(phot.ray().pos(), grid, &mut light_map);
        *cell_rec.1.emissions_mut() += phot.weight();
        let mut env = verse
            .mats()
            .map()
            .get(cell_rec.0.mat())
            .expect("Invalid material name.")
            .optics()
            .env(*phot.wavelength());

        let mut num_loops = 0;
        loop {
            num_loops += 1;
            if num_loops >= MAX_LOOPS {
                warn!(
                    "Photon prematurely killed as number of loops exceeded {}",
                    MAX_LOOPS
                );
            }

            if *phot.weight() < ROULETTE {
                if rng.gen_range(0.0_f64, 1.0) <= ROULETTE {
                    *phot.weight_mut() /= ROULETTE;
                } else {
                    break;
                }
            }

            let scat_dist = -(rng.gen_range(0.0_f64, 1.0)).ln() / env.inter_coeff();
            let cell_dist = cell_rec
                .0
                .bound()
                .dist(phot.ray())
                .expect("Unable to determine boundary distance.");
            let inter_dist = cell_rec.0.inter_dist(phot.ray());

            match Hit::new(scat_dist, cell_dist, inter_dist, bump_dist) {
                Hit::Scattering(dist) => {
                    *cell_rec.1.dist_travelled_mut() += dist;
                    phot.ray_mut().travel(dist);

                    *cell_rec.1.scatters_mut() += phot.weight();
                    phot.ray_mut().rotate(
                        henyey_greenstein(&mut rng, *env.asym()),
                        rng.gen_range(0.0, 2.0 * PI),
                    );

                    *cell_rec.1.absorptions_mut() += env.albedo() * phot.weight();
                    *phot.weight_mut() *= env.albedo();

                    if !shifted && rng.gen_range(0.0, 1.0) <= env.shift_prob() {
                        *cell_rec.1.shifts_mut() += phot.weight();
                        shifted = true;
                    }
                }
                Hit::Cell(dist) => {
                    let dist = dist + bump_dist;
                    *cell_rec.1.dist_travelled_mut() += dist;
                    phot.ray_mut().travel(dist);

                    if !grid.bound().contains(phot.ray().pos()) {
                        break;
                    }

                    cell_rec = cell_and_record(phot.ray().pos(), grid, &mut light_map);
                }
                Hit::Interface(dist) => {
                    hit_interface(
                        &mut rng,
                        &mut phot,
                        &mut cell_rec,
                        &mut env,
                        dist,
                        bump_dist,
                        verse.mats(),
                    );

                    if !cell_rec.0.bound().contains(phot.ray().pos()) {
                        // TODO: This should be able to be removed.
                        if !grid.bound().contains(phot.ray().pos()) {
                            break;
                        }

                        warn!("It happened!");
                        cell_rec = cell_and_record(phot.ray().pos(), grid, &mut light_map);
                    }
                }
                Hit::InterfaceCell(dist) => {
                    hit_interface(
                        &mut rng,
                        &mut phot,
                        &mut cell_rec,
                        &mut env,
                        dist,
                        bump_dist,
                        verse.mats(),
                    );

                    if !grid.bound().contains(phot.ray().pos()) {
                        break;
                    }

                    cell_rec = cell_and_record(phot.ray().pos(), grid, &mut light_map);
                }
            }
        }
    }

    pb.finish_with_message("Photon loop complete.");

    light_map
}

/// Retrieve a reference to the current cell, and corresponding record, that a point belongs to.
#[inline]
#[must_use]
fn cell_and_record<'a>(
    pos: &Point3<f64>,
    grid: &'a Regular,
    light_map: &'a mut LightMap,
) -> (&'a Cell<'a>, &'a mut Record) {
    let mins = grid.bound().mins();
    let maxs = grid.bound().maxs();
    let shape = grid.cells().shape();

    let id: Vec<usize> = pos
        .iter()
        .zip(mins.iter().zip(maxs.iter()))
        .zip(shape)
        .map(|((p, (min, max)), n)| index(*p, *min, *max, *n))
        .collect();
    let index = (
        *id.get(0).expect("Missing index."),
        *id.get(1).expect("Missing index."),
        *id.get(2).expect("Missing index."),
    );

    let cell = grid.cells().get(index).expect("Invalid grid index.");
    let rec = light_map
        .recs_mut()
        .get_mut(index)
        .expect("Invalid record index.");

    assert!(cell.bound().contains(pos));

    (cell, rec)
}

/// Determine the index corresponding to a given resolution.
#[inline]
#[must_use]
pub fn index(x: f64, min: f64, max: f64, res: usize) -> usize {
    (((x - min) / (max - min)) * res as f64) as usize
}

/// Current cell and record information.
pub struct Info<'a> {
    /// Reference to the currently occupied cell.
    cell: &'a Cell<'a>,
    /// Reference to the corresponding record.
    rec: &'a mut Record,
}

impl<'a> Info<'a> {
    access!(cell, &'a Cell<'a>);
    access!(rec, &'a mut Record);

    /// Construct a new instance.
    pub fn new(cell: &'a Cell, rec: &'a mut Record) -> Self {
        Self { cell, rec }
    }
}

/// Perform an interface hit event.
fn hit_interface(
    rng: &mut ThreadRng,
    phot: &mut Photon,
    cell_rec: &mut (&Cell, &mut Record),
    env: &mut Environment,
    dist: f64,
    bump_dist: f64,
    mats: &Set<Material>,
) {
    let (_dist, inside, norm, inter) = cell_rec
        .0
        .inter_dist_inside_norm_inter(phot.ray())
        .expect("Failed to observe interface within cell.");

    let next_mat = if inside {
        inter.out_mat()
    } else {
        inter.in_mat()
    };
    let next_env = mats
        .map()
        .get(next_mat)
        .expect("Invalid material name")
        .optics()
        .env(*phot.wavelength());

    let n_curr = *env.ref_index();
    let n_next = *next_env.ref_index();

    let crossing = Crossing::new(phot.ray().dir(), &norm, n_curr, n_next);

    if rng.gen_range(0.0, 1.0) <= *crossing.ref_prob() {
        let effective_dist = (dist - bump_dist).max(MIN_POSITIVE);
        *cell_rec.1.dist_travelled_mut() += effective_dist;
        phot.ray_mut().travel(effective_dist);
        *phot.ray_mut().dir_mut() = *crossing.ref_dir();
    } else {
        let effective_dist = dist + bump_dist;
        *cell_rec.1.dist_travelled_mut() += effective_dist;
        phot.ray_mut().travel(effective_dist);
        *phot.ray_mut().dir_mut() = crossing
            .trans_dir()
            .expect("Failed to determine transmission direction.");

        *env = next_env;
    }
}
