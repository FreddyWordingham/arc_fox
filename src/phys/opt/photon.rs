//! Optical photon structure.

use crate::rt::Ray;
use contracts::pre;
use nalgebra::{Rotation3, Unit, Vector3};

/// Optical photon structure.
pub struct Photon {
    /// Ray of travel.
    ray: Ray,
    /// Wavelength.
    wavelength: f64,
}

impl Photon {
    /// Construct a new instance.
    #[pre(wavelength > 0.0)]
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
    #[pre(dist > 0.0)]
    pub fn travel(&mut self, dist: f64) {
        self.ray.pos += self.ray.dir.as_ref() * dist;
    }

    /// Pitch towards the z-axis and then roll around previous direction.
    #[pre(self.ray.dir.z.abs() != 1.0)]
    pub fn rotate(&mut self, pitch: f64, roll: f64) {
        let pitch_axis = Unit::new_normalize(self.ray.dir.cross(&Vector3::z_axis()));
        let pitch_rot = Rotation3::from_axis_angle(&pitch_axis, pitch);

        let roll_rot = Rotation3::from_axis_angle(&self.ray.dir, roll);

        self.ray.dir = roll_rot * pitch_rot * self.ray.dir;
    }
}
