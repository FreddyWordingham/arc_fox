//! Light structure.

use crate::{rng::Distribution, rt::Emitter};
use contracts::pre;

/// Photon emission structure.
pub struct Light {
    /// Emission surface.
    emit: Emitter,
    /// Wavelength distribution.
    dist: Distribution,
    /// Power. [J/s]
    power: f64,
}

impl Light {
    /// Construct a new instance.
    #[pre(power > 0.0)]
    pub fn new(emit: Emitter, dist: Distribution, power: f64) -> Self {
        Self { emit, dist, power }
    }
}
