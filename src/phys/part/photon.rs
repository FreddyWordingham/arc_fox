//! Photon implementation.

use crate::{access, geom::Ray};

/// Photon structure.
#[derive(Debug)]
pub struct Photon {
    /// Statistical weight.
    weight: f64,
    /// Wavelength [m].
    wavelength: f64,
    /// Power [J/s].
    power: f64,
    /// Ray of travel.
    ray: Ray,
}

impl Photon {
    access!(weight, f64);
    access!(wavelength, f64);
    access!(power, f64);
    access!(ray, ray_mut, Ray);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(wavelength: f64, power: f64, ray: Ray) -> Self {
        Self {
            weight: 1.0,
            wavelength,
            power,
            ray,
        }
    }
}
