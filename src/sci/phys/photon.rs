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
