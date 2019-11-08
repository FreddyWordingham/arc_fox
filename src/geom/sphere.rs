//! Sphere structure.

use super::{Aabb, Collision};
use crate::rt::{Ray, Traceable};
use contracts::pre;
use nalgebra::{Point3, Unit, Vector3};

/// Spherical geometry.
pub struct Sphere {
    /// Central position of the sphere.
    pos: Point3<f64>,
    /// Radius of the sphere.
    rad: f64,
}

impl Sphere {
    /// Construct a new instance.
    #[pre(rad > 0.0)]
    pub fn new(pos: Point3<f64>, rad: f64) -> Self {
        Self { pos, rad }
    }
}

impl Traceable for Sphere {
    fn hit(&self, ray: &Ray) -> bool {
        let oc = ray.pos - self.pos;
        let b = ray.dir.dot(&oc).powi(2);
        let c = oc.magnitude().powi(2) - self.rad.powi(2);

        b >= c
    }

    fn dist(&self, ray: &Ray) -> Option<f64> {
        let oc = ray.pos - self.pos;
        let a = -ray.dir.dot(&oc);
        let b = ray.dir.dot(&oc).powi(2);
        let c = oc.magnitude().powi(2) - self.rad.powi(2);

        if b < c {
            return None;
        }

        let det_sqrt = (b - c).sqrt();

        let d0 = a + det_sqrt;
        let d1 = a - det_sqrt;

        if (d0 < 0.0) && (d1 < 0.0) {
            return None;
        }

        if d0 < 0.0 {
            if d1 >= 0.0 {
                return Some(d1);
            }

            return None;
        }

        if d1 < 0.0 {
            if d0 >= 0.0 {
                return Some(d0);
            }

            return None;
        }

        if d0 < d1 {
            return Some(d0);
        }

        Some(d1)
    }

    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        if let Some(dist) = self.dist(ray) {
            let p = ray.pos + (ray.dir.as_ref() * dist);
            let norm = Unit::new_normalize(p - &self.pos);

            return Some((dist, norm));
        }

        None
    }
}

impl Collision for Sphere {
    fn contains(&self, aabb: &Aabb) -> bool {
        self.rad >= nalgebra::distance(&self.pos, &aabb.closest_point(&self.pos))
    }
}
