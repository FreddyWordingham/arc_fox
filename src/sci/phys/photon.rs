//! Photon structure.

use crate::sci::math::rt::Ray;

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
}
