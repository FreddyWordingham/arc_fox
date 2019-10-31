//! Geometric plane structure.

use super::{Ray, Traceable};
use nalgebra::{Point3, Unit, Vector3};
use serde::{Deserialize, Serialize};

/// Parallel ray catch value.
const EPSILON: f64 = 1.0e-6;

/// Infinite plane geometry.
#[derive(Debug, Serialize, Deserialize)]
pub struct Plane {
    /// Position on the plane.
    pos: Point3<f64>,
    /// Normal.
    norm: Unit<Vector3<f64>>,
}

impl Plane {
    /// Construct a new instance.
    pub fn new(pos: Point3<f64>, norm: Unit<Vector3<f64>>) -> Self {
        Self { pos, norm }
    }

    /// Reference the normal of the plane.
    pub fn norm(&self) -> &Unit<Vector3<f64>> {
        &self.norm
    }
}

impl Traceable for Plane {
    fn intersect(&self, ray: &Ray) -> bool {
        let d = self.norm.dot(&ray.dir);

        if d.abs() > EPSILON {
            let po = self.pos - ray.pos;
            let dist = po.dot(&self.norm) / d;

            return dist >= 0.0;
        }

        false
    }

    fn dist(&self, ray: &Ray) -> Option<f64> {
        let d = self.norm.dot(&ray.dir);

        if d.abs() > EPSILON {
            let rp = self.pos - ray.pos;
            let dist = rp.dot(&self.norm) / d;

            if dist < 0.0 {
                return None;
            }

            return Some(dist);
        }

        None
    }

    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        let d = self.norm.dot(&ray.dir);

        if d.abs() > EPSILON {
            let po = self.pos - ray.pos;
            let dist = po.dot(&self.norm) / d;

            if dist < 0.0 {
                return None;
            }

            return Some((dist, self.norm));
        }

        None
    }
}
