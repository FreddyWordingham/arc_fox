//! Light-source structure.

use crate::{
    access,
    dom::{Name, Set},
    geom::{Emit, Mesh},
    phys::{Photon, Spectrum},
};
use attr::json;
use rand::rngs::ThreadRng;
use std::fmt::{Display, Formatter, Result};

/// Light structure implementation.
#[json]
pub struct Light {
    /// Emission surface.
    surf: Name,
    /// Emission spectrum.
    spec: Spectrum,
    /// Power. [J/s]
    power: f64,
}

impl Light {
    access!(surf, Name);
    access!(spec, Spectrum);
    access!(power, f64);

    /// Construct a new instance.
    pub fn new(surf: Name, spec: Spectrum, power: f64) -> Self {
        assert!(power > 0.0);

        Self { surf, spec, power }
    }

    /// Emit a new photon.
    pub fn emit(&self, rng: &mut ThreadRng, total_phot: u64, meshes: &Set<Mesh>) -> Photon {
        Photon::new(
            self.spec.sample(rng),
            self.power / total_phot as f64,
            meshes
                .map()
                .get(&self.surf)
                .expect("Invalid mesh name.")
                .cast(rng),
        )
    }
}

impl Display for Light {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(fmt, "power: {}w,\tsurf: {}\t", self.power, self.surf)
    }
}
