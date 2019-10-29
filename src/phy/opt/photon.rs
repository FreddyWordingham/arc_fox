//! Optical photon structure.

use crate::geom::Ray;
use contracts::pre;

/// Optical photon structure.
pub struct Photon {
    /// Ray of travel.
    ray: Ray,
    /// Wavelength.
    wavelength: f64,
}

impl Photon {
    /// Construct a new instance.
    #[pre(0.0 < wavelength)]
    pub fn new(ray: Ray, wavelength: f64) -> Self {
        Self { ray, wavelength }
    }

    /// Reference the ray.
    pub fn ray(&self) -> &Ray {
        &self.ray
    }

    /// Get the wavelength.
    pub fn wavelength(&self) -> f64 {
        self.wavelength
    }
}
