//! Emitter trait.

use super::Ray;
use contracts::post;
use nalgebra::{Point3, Unit, Vector3};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

/// Trait implemented by geometries which can be randomly sampled for rays.
pub trait Emitter {
    fn emit(&self, rng: &mut ThreadRng) -> Ray;
}

impl Emitter for Point3<f64> {
    #[post((ret.dir.magnitude() - 1.0).abs() < 1.0e-9)]
    fn emit(&self, rng: &mut ThreadRng) -> Ray {
        let theta = rng.gen_range(0.0, 2.0 * PI);
        let z = rng.gen_range(-1.0, 1.0);

        Ray::new(
            self,
            Unit::new_normalize(Vector3::new(
                (1.0f64 - (z * z)).sqrt() * theta.cos(),
                (1.0f64 - (z * z)).sqrt() * theta.sin(),
                z,
            )),
        )
    }
}

impl Emitter for (Point3<f64>, Unit<Vector3<f64>>, f64) {
    #[post((ret.magnitude() - 1.0).abs() < 1.0e-9)]
    fn emit(&self, rng: &mut ThreadRng) -> Ray {
        let pitch = rng.gen_range(self.2, 1.0).acos();
        let roll = rng.gen_range(0.0, 2.0 * PI);

        let mut ray = Ray::new(self.0, self.1);
        ray.rotate(pitch, roll);

        ray
    }
}
