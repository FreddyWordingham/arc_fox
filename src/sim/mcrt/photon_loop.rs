//! MCRT photon-loop functions.

use crate::{
    sim::mcrt::LightMap,
    util::list::dimension::Cartesian::{X, Y, Z},
    world::{parts::Light, Universe},
};

/// Start a single-threaded photon loop.
pub fn start(thread_id: usize, _light: &Light, universe: &Universe) -> LightMap {
    let shape: [usize; 3] = [
        universe.grid().cells().shape()[X as usize],
        universe.grid().cells().shape()[Y as usize],
        universe.grid().cells().shape()[Z as usize],
    ];
    let lightmap = LightMap::new(shape, universe.grid().cell_vol());

    println!("Photon loop using thread: {}", thread_id);

    lightmap
}
