//! Optical photon structure.

use crate::rt::Ray;
use contracts::pre;

/// Optical photon structure.
pub struct Photon {
    /// Statistical weight.
    weight: f64,
    /// Ray of travel.
    ray: Ray,
    /// Wavelength.
    wavelength: f64,
}

impl Photon {
    /// Construct a new instance.
    #[pre(wavelength > 0.0)]
    pub fn new(ray: Ray, wavelength: f64) -> Self {
        Self {
            weight: 1.0,
            ray,
            wavelength,
        }
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
    #[pre(dist > 0.0)]
    pub fn travel(&mut self, dist: f64) {
        self.ray.travel(dist);
    }

    /// Pitch towards the z-axis and then roll around previous direction.
    #[pre(self.ray.dir.z.abs() != 1.0)]
    pub fn rotate(&mut self, pitch: f64, roll: f64) {
        self.ray.rotate(pitch, roll);
    }
}
