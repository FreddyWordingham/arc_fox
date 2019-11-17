//! Photon structure.

use crate::rt::Ray;
use contracts::{post, pre};

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
    #[post(ret > 0.0)]
    #[post(ret <= 1.0)]
    pub fn weight(&self) -> f64 {
        self.weight
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
    #[post((ret.dir().magnitude() - 1.0).abs() < 1.0e-6)]
    pub fn ray(&self) -> &Ray {
        &self.ray
    }
}
