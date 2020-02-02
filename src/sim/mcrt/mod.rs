//! Monte-Carlo radiative transfer simulation sub-module.

pub mod hit;
pub mod light_map;
pub mod record;

pub use self::hit::*;
pub use self::light_map::*;
pub use self::record::*;

use crate::{
    dom::{Name, Regular},
    uni::Verse,
    util::bar,
};
use rand::thread_rng;

/// Generate a lightmap for a given setup.
#[inline]
#[must_use]
pub fn run(name: &Name, num_phot: u64, verse: &Verse, grid: &Regular) -> LightMap {
    let pb = bar("Photon loop", num_phot);
    let mut rng = thread_rng();

    let light = &verse.lights().map()[name];
    let light_map = LightMap::new(grid.res(), grid.cell_vol());
    for _ in 0..num_phot {
        let _phot = light.emit(&mut rng, num_phot, verse.meshes());
        pb.inc(1);
    }

    pb.finish_with_message("Photon loop complete.");

    light_map
}
