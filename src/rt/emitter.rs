//! Ray emitter trait.

use rand::{rngs::ThreadRng, Rng};
use super::Ray;
use nalgebra::{Point3, Unit, Vector3};
use std::f64::consts::PI;

/// Types implementing this trait can emit rays.
pub trait Emitter {
    /// Emit a new ray.
    fn emit(&self, rng: &mut ThreadRng) -> Ray;
}

impl Emitter for Point3<f64> {
    fn emit(&self, rng: &mut ThreadRng) -> Ray {
        let theta = rng.gen_range(0.0, 2.0 * PI);
        let z = rng.gen_range(-1.0f64, 1.0);

        Ray::new(
            *self,
            Unit::new_normalize(Vector3::new(
                (1.0 - z.powi(2)).sqrt() * theta.cos(),
                (1.0 - z.powi(2)).sqrt() * theta.sin(),
                z,
            )),
        )
    }
}
