//! Plane structure.

use super::EPSILON;
use crate::rt::{Ray, Traceable};
use nalgebra::{Point3, Unit, Vector3};

/// Plane geometry.
pub struct Plane {
    /// Point on the plane.
    pos: Point3<f64>,
    /// Normal.
    norm: Unit<Vector3<f64>>,
}

impl Plane {
    /// Construct a new instance.
    pub fn new(pos: Point3<f64>, norm: Unit<Vector3<f64>>) -> Self {
        Self { pos, norm }
    }
}

impl Traceable for Plane {
    fn hit(&self, ray: &Ray) -> bool {
        self.dist(ray).is_some()
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
        if let Some(dist) = self.dist(ray) {
            return Some((dist, self.norm));
        }

        None
    }
}
