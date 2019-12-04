//! Photon structure.

use crate::sci::math::rt::Ray;
use contracts::{post, pre};
// use nalgebra::{Unit, Vector3};

/// Photon structure implementation.
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
    /// Construct a new instance.
    #[pre(wavelength > 0.0)]
    #[pre(power > 0.0)]
    pub fn new(wavelength: f64, power: f64, ray: Ray) -> Self {
        Self {
            weight: 1.0,
            wavelength,
            power,
            ray,
        }
    }

    /// Get the weight.
    #[post(ret > 0.0 && ret <= 1.0)]
    pub fn weight(&self) -> f64 {
        self.weight
    }

    /// Multiply the weight of the photon.
    #[pre(x > 0.0)]
    #[post(self.weight > 0.0)]
    pub fn multiply_weight(&mut self, x: f64) {
        self.weight *= x;
    }

    /// Get the wavelength.
    #[post(ret > 0.0)]
    pub fn wavelength(&self) -> f64 {
        self.wavelength
    }

    /// Get the power.
    #[post(ret > 0.0)]
    pub fn power(&self) -> f64 {
        self.power
    }

    /// Reference the ray.
    pub const fn ray(&self) -> &Ray {
        &self.ray
    }

    /// Move along the direction the given distance.
    #[pre(dist > 0.0)]
    pub fn travel(&mut self, dist: f64) {
        self.ray.travel(dist);
    }

    /// Pitch towards the z-axis and then roll around previous direction.
    pub fn rotate(&mut self, pitch: f64, roll: f64) {
        self.ray.rotate(pitch, roll);
    }
}
