//! Photon structure.

use crate::{access, sci::math::rt::Ray};

/// Photon as a particle representation.
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
    access!(ray, Ray);

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

    /// Move the photon forward the given distance.
    #[inline]
    pub fn travel(&mut self, dist: f64) {
        self.ray.travel(dist);
    }

    /// Rotate the photon with a given pitch and subsequent roll manoeuvre.
    #[inline]
    pub fn rotate(&mut self, pitch: f64, roll: f64) {
        self.ray.rotate(pitch, roll);
    }
}
