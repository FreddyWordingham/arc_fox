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
    pow: f64,
}

impl Light {
    /// Construct a new instance.
    #[pre(pow > 0.0)]
    pub fn new(emit: Emitter, dist: Distribution, pow: f64) -> Self {
        Self { emit, dist, pow }
    }
}
