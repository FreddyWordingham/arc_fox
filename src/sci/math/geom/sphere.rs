//! Sphere geometry structure.

use crate::sci::math::{
    geom::{Aabb, Collide},
    rt::{Ray, Trace},
};
use nalgebra::{Point3, Unit, Vector3};
use std::f64::consts::PI;

/// Sphere geometry.
#[derive(Clone)]
pub struct Sphere {
    /// Central point.
    pub pos: Point3<f64>,
    /// Radius.
    pub rad: f64,
}

impl Sphere {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(pos: Point3<f64>, rad: f64) -> Self {
        Self { pos, rad }
    }

    /// Calculate the surface area.
    #[inline]
    #[must_use]
    pub fn area(&self) -> f64 {
        4.0 * PI * self.rad.powi(2)
    }

    /// Calculate the volume.
    #[inline]
    #[must_use]
    pub fn vol(&self) -> f64 {
        4.0 / 3.0 * PI * self.rad.powi(3)
    }

    /// Determine if the given point if contained.
    #[inline]
    #[must_use]
    pub fn contains(&self, p: &Point3<f64>) -> bool {
        nalgebra::distance_squared(&self.pos, p) <= self.rad.powi(2)
    }

    /// Determine the intersection distances along a ray's direction.
    #[must_use]
    fn intersections(&self, ray: &Ray) -> Option<(f64, f64)> {
        let m = ray.pos - self.pos;
        let b = m.dot(&ray.dir);
        let c = (m.dot(&m)) - self.rad.powi(2);

        if c > 0.0 && b > 0.0 {
            return None;
        }

        let disc = b.powi(2) - c;
        if disc <= 0.0 {
            return None;
        }

        let disc_sqrt = disc.sqrt();
        Some((-b - disc_sqrt, -b + disc_sqrt))
    }
}

impl Collide for Sphere {
    #[inline]
    #[must_use]
    fn bounding_box(&self) -> Aabb {
        Aabb::new_centred(&self.pos, &Vector3::new(self.rad, self.rad, self.rad))
    }

    #[inline]
    #[must_use]
    fn overlap(&self, aabb: &Aabb) -> bool {
        aabb.dist_sq_from_point(&self.pos) <= self.rad.powi(2)
    }
}

impl Trace for Sphere {
    #[must_use]
    fn hit(&self, ray: &Ray) -> bool {
        self.intersections(ray).is_some()
    }

    #[must_use]
    fn dist(&self, ray: &Ray) -> Option<f64> {
        if let Some((min, max)) = self.intersections(ray) {
            if min > 0.0 {
                return Some(min);
            } else if max > 0.0 {
                return Some(max);
            }
        }

        None
    }

    #[must_use]
    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        if let Some(dist) = self.dist(ray) {
            let mut hit = ray.clone();
            hit.travel(dist);
            return Some((dist, Unit::new_normalize(hit.pos - self.pos)));
        }

        None
    }

    #[must_use]
    fn dist_inside(&self, ray: &Ray) -> Option<(f64, bool)> {
        if let Some((min, max)) = self.intersections(ray) {
            if min > 0.0 {
                return Some((min, false));
            }
            if max > 0.0 {
                return Some((max, true));
            }
        }

        None
    }

    #[must_use]
    fn dist_inside_norm(&self, ray: &Ray) -> Option<(f64, bool, Unit<Vector3<f64>>)> {
        if let Some((dist, inside)) = self.dist_inside(ray) {
            let mut hit = ray.clone();
            hit.travel(dist);
            return Some((dist, inside, Unit::new_normalize(hit.pos - self.pos)));
        }

        None
    }
}
