//! Aperture structure.

use crate::sci::math::rt::{Emit, Ray};
use contracts::{post, pre};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::{FRAC_PI_2, PI};

/// Aperture structure implementation.
#[derive(Debug)]
pub struct Aperture {
    /// Ray.
    ray: Ray,
    /// Numerical aperture.
    na: f64,
}

impl Aperture {
    /// Construct a new instance.
    #[pre(na > 0.0)]
    #[pre(na < FRAC_PI_2)]
    pub fn new(ray: Ray, na: f64) -> Self {
        Self { ray, na }
    }
}

impl Emit for Aperture {
    #[post((ret.dir().magnitude() - 1.0).abs() < 1.0e-6)]
    fn emit(&self, rng: &mut ThreadRng) -> Ray {
        let pitch = rng.gen_range(self.na.cos(), 1.0).acos();
        let roll = rng.gen_range(0.0, 2.0 * PI);

        let mut ray = self.ray.clone();
        ray.rotate(pitch, roll);

        ray
    }
}
