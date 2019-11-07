//! Ray emitter enumeration.

use super::Ray;
use contracts::pre;
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
    /// Spotlight.
    Spotlight {
        /// Position of emission.
        pos: Point3<f64>,
        /// General direction of emission.
        dir: Unit<Vector3<f64>>,
        /// Cosine of the numerical aperture.
        cos_na: f64,
    },
}

impl Emitter {
    /// Construct a new point emitter.
    pub fn new_point(pos: Point3<f64>) -> Self {
        Self::Point { pos }
    }

    #[pre(na > 0.0)]
    #[pre(na < PI)]
    pub fn new_spotlight(pos: Point3<f64>, dir: Unit<Vector3<f64>>, na: f64) -> Self {
        Self::Spotlight {
            pos,
            dir,
            cos_na: na.cos(),
        }
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
            Emitter::Spotlight { pos, dir, cos_na } => {
                let pitch = rng.gen_range(cos_na, 1.0).acos();
                let roll = rng.gen_range(0.0, 2.0 * PI);

                let mut ray = Ray::new(*pos, *dir);
                ray.rotate(pitch, roll);

                ray
            }
        }
    }
}
