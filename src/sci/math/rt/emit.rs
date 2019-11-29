//! Emit trait.

use super::Ray;
use contracts::post;
use nalgebra::{Point3, Unit, Vector3};
use rand::{rngs::ThreadRng, Rng};
use std::{f64::consts::PI, fmt::Debug};

/// Emit trait implementation.
/// Type implementing this trait can be randomly sampled for rays.
pub trait Emit: Debug + Sync {
    /// Emit a new ray.
    fn emit(&self, rng: &mut ThreadRng) -> Ray;
}

impl Emit for Point3<f64> {
    #[post((ret.dir().magnitude_squared() - 1.0).abs() < 1.0e-6)]
    fn emit(&self, rng: &mut ThreadRng) -> Ray {
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
