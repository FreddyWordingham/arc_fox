//! Light-source structure.

use crate::sci::{
    math::rt::Emit,
    phys::{Photon, Spectrum},
};
use contracts::pre;
use rand::rngs::ThreadRng;

/// Light structure implementation.
#[derive(Debug)]
pub struct Light {
    /// Emission surface.
    surf: Box<dyn Emit>,
    /// Emission spectrum.
    spec: Spectrum,
    /// Power. [J/s]
    power: f64,
}

impl Light {
    /// Construct a new instance.
    #[pre(power > 0.0)]
    pub fn new(surf: Box<dyn Emit>, spec: Spectrum, power: f64) -> Self {
        Self { surf, spec, power }
    }

    /// Emit a new photon.
    #[pre(total_phot > 0)]
    pub fn emit(&self, rng: &mut ThreadRng, total_phot: u64) -> Photon {
        Photon::new(
            self.spec.sample(rng),
            self.power / total_phot as f64,
            self.surf.emit(rng),
        )
    }
}
