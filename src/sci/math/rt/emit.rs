//! Emit trait.

use crate::sci::math::rt::Ray;
use nalgebra::{Point3, Unit, Vector3};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

/// Trait of geometries which can emit 'Ray's.
pub trait Emit {
    /// Cast a new Ray.
    fn cast(&self, rng: &mut ThreadRng) -> Ray;
}

impl Emit for Point3<f64> {
    fn cast(&self, rng: &mut ThreadRng) -> Ray {
        let theta = rng.gen_range(0.0, 2.0 * PI);
        let z = rng.gen_range(-1.0, 1.0);

        Ray::new(
            *self,
            Unit::new_normalize(Vector3::new(
                (1.0f64 - (z * z)).sqrt() * theta.cos(),
                (1.0f64 - (z * z)).sqrt() * theta.sin(),
                z,
            )),
        )
    }
}
