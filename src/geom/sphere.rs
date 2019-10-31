//! Geometic sphere structure.

use super::{Aabb, Collidable, Ray, Traceable};
use contracts::pre;
use nalgebra::{Isometry3, Point3, Unit, Vector3};
use std::f64::consts::PI;

/// Geometry describing a sphere.
#[derive(Debug)]
pub struct Sphere {
    /// Central point.
    centre: Point3<f64>,
    /// Radius.
    rad: f64,
}

impl Sphere {
    /// Construct a new instance.
    #[pre(rad > 0.0)]
    pub fn new(centre: Point3<f64>, rad: f64) -> Self {
        Self { centre, rad }
    }

    /// Reference the central point.
    pub fn centre(&self) -> &Point3<f64> {
        &self.centre
    }

    /// Get the radius.
    pub fn rad(&self) -> f64 {
        self.rad
    }

    /// Calculate the area.
    pub fn area(&self) -> f64 {
        4.0 * PI * self.rad.powi(2)
    }

    /// Calculate the volume.
    pub fn vol(&self) -> f64 {
        4.0 / 3.0 * PI * self.rad.powi(3)
    }

    /// Determine if the given point is contained.
    /// Points lying exactly at the surface are considered contained.
    pub fn contains(&self, point: Point3<f64>) -> bool {
        (point - self.centre).magnitude() <= self.rad
    }
}

impl Traceable for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        let oc = ray.pos - self.centre;
        let b = ray.dir.dot(&oc).powi(2);
        let c = oc.magnitude().powi(2) - self.rad.powi(2);

        b >= c
    }

    fn dist(&self, ray: &Ray) -> Option<f64> {
        let oc = ray.pos - self.centre;
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
            let norm = Unit::new_normalize(p - self.centre());

            return Some((dist, norm));
        }

        None
    }
}
