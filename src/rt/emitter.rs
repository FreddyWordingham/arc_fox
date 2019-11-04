//! Ray emitter enumeration.

use super::Ray;
use nalgebra::{Point3, Unit, Vector3};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

/// Ray emission structure.
pub enum Emitter {
    /// Point source.
    Point {
        /// Position of emission.
        pos: Point3<f64>,
    },
}

impl Emitter {
    /// Construct a new point emitter.
    pub fn new_point(pos: Point3<f64>) -> Self {
        Self::Point { pos }
    }

    /// Emit a new ray.
    pub fn emit(&self, rng: &mut ThreadRng) -> Ray {
        match self {
            Emitter::Point { pos } => {
                let theta = rng.gen_range(0.0, 2.0 * PI);
                let z = rng.gen_range(-1.0, 1.0);

                Ray::new(
                    *pos,
                    Unit::new_normalize(Vector3::new(
                        (1.0f64 - (z * z)).sqrt() * theta.cos(),
                        (1.0f64 - (z * z)).sqrt() * theta.sin(),
                        z,
                    )),
                )
            }
        }
    }
}
