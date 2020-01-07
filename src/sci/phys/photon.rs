//! Photon structure.

use crate::sci::math::rt::Ray;

/// Photon as a particle representation.
pub struct Photon {
    /// Statistical weight.
    pub weight: f64,
    /// Wavelength [m].
    pub wavelength: f64,
    /// Power [J/s].
    pub power: f64,
    /// Ray of travel.
    pub ray: Ray,
}

impl Photon {
    /// Construct a new instance.
    #[inline]
    pub fn new(wavelength: f64, power: f64, ray: Ray) -> Self {
        if wavelength <= 0.0 {
            panic!("Photon wavelengths must be positive: {}", wavelength);
        }
        if power <= 0.0 {
            panic!("Photon powers must be positive: {}", power);
        }

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
