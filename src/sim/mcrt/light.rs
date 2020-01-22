//! Light source structure.

use crate::sci::{
    math::rt::Emit,
    phys::{Photon, Spectrum},
};
use rand::rngs::ThreadRng;

/// Light source structure used to emit photons.
pub struct Light {
    /// Spectrum.
    spec: Spectrum,
    /// Emission surface.
    surf: Box<dyn Emit>,
}

impl Light {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(spec: Spectrum, surf: Box<dyn Emit>) -> Self {
        Self { spec, surf }
    }

    /// Emit a new photon.
    #[inline]
    #[must_use]
    pub fn emit(&self, rng: &mut ThreadRng, power: f64) -> Photon {
        let w = self.spec.sample(rng);
        let ray = self.surf.cast(rng);

        Photon::new(w, power, ray)
    }
}
