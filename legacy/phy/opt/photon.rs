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

    /// Move along the direction the given distance.
    #[pre(0.0 < dist)]
    pub fn travel(&mut self, dist: f64) {
        self.ray.travel(dist);
    }

    /// Pitch towards the z-axis and then roll around previous direction.
    pub fn rotate(&mut self, pitch: f64, roll: f64) {
        self.ray.rotate(pitch, roll)
    }
}
