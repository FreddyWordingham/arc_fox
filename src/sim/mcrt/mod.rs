//! Monte-Carlo radiative transfer simulation sub-module.

pub mod hit;
pub mod light_map;
pub mod record;

pub use self::hit::*;
pub use self::light_map::*;
pub use self::record::*;

use crate::{
    dom::{Cell, Name, Regular},
    uni::Verse,
    util::bar,
};
use nalgebra::Point3;
use rand::thread_rng;

/// Generate a lightmap for a given setup.
#[inline]
#[must_use]
pub fn run(name: &Name, num_phot: u64, verse: &Verse, grid: &Regular) -> LightMap {
    let pb = bar("Photon loop", num_phot);
    let mut rng = thread_rng();

    let light = &verse.lights().map()[name];
    let mut light_map = LightMap::new(grid.res(), grid.cell_vol());
    for _ in 0..num_phot {
        pb.inc(1);

        let phot = light.emit(&mut rng, num_phot, verse.meshes());

        let cell_rec = cell_and_record(phot.ray().pos(), grid, &mut light_map);
        *cell_rec.1.emissions_mut() += phot.weight();

        // while grid.bound().contains(phot.ray().pos()) {
        //     phot.travel(1.0e-3);
        // }
    }

    pb.finish_with_message("Photon loop complete.");

    light_map
}

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
    let index = (id[0], id[1], id[2]);

    let cell = &grid.cells()[index];
    let rec = &mut light_map.recs_mut()[index];

    assert!(cell.bound().contains(pos));

    (cell, rec)
}

pub fn index(x: f64, min: f64, max: f64, res: usize) -> usize {
    (((x - min) / (max - min)) * res as f64) as usize
}
