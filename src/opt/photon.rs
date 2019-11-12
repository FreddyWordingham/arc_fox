//! Optical photon structure.

use crate::rt::Ray;
use contracts::{post, pre};
use nalgebra::{Unit, Vector3};
use std::f64::consts::PI;

/// Optical photon structure.
pub struct Photon {
    /// Statistical weight.
    weight: f64,
    /// Power [J/s].
    power: f64,
    /// Ray of travel.
    ray: Ray,
    /// Wavelength [m].
    wavelength: f64,
}

impl Photon {
    /// Construct a new instance.
    #[pre(wavelength > 0.0)]
    #[pre(power > 0.0)]
    pub fn new(ray: Ray, power: f64, wavelength: f64) -> Self {
        Self {
            weight: 1.0,
            power,
            ray,
            wavelength,
        }
    }

    /// Get the weight.
    #[post(ret > 0.0)]
    #[post(ret <= 1.0)]
    pub fn weight(&self) -> f64 {
        self.weight
    }

    /// Reference the ray.
    #[post((ret.dir.magnitude() - 1.0).abs() < 1.0e-6)]
    pub fn ray(&self) -> &Ray {
        &self.ray
    }

    /// Get the power.
    #[post(ret > 0.0)]
    pub fn power(&self) -> f64 {
        self.power
    }

    /// Get the wavelength.
    #[post(ret > 0.0)]
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
    #[pre(pitch > 0.0)]
    #[pre(pitch < PI)]
    #[pre(pitch > 0.0)]
    #[pre(pitch < (2.0 * PI))]
    #[post((self.ray.dir.magnitude() - 1.0).abs() < 1.0e-6)]
    pub fn rotate(&mut self, pitch: f64, roll: f64) {
        self.ray.rotate(pitch, roll);
    }

    /// Set direction manually.
    #[pre((dir.magnitude() - 1.0).abs() < 1.0e-6)]
    #[post((self.ray.dir.magnitude() - 1.0).abs() < 1.0e-6)]
    pub fn set_dir(&mut self, dir: Unit<Vector3<f64>>) {
        self.ray.dir = dir;
    }

    /// Multiply the weight of the photon.
    #[pre(x > 0.0)]
    #[post(self.weight > 0.0)]
    pub fn multiply_weight(&mut self, x: f64) {
        self.weight *= x;
    }
}
