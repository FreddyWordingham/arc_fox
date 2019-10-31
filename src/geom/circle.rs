//! Geometric circle structure.

use super::{Ray, Traceable};
use contracts::pre;
use nalgebra::{Point3, Unit, Vector3};
use std::f64::consts::PI;

/// Parallel ray catch value.
const EPSILON: f64 = 1.0e-6;

/// Geometry describing a radius around a point on a plane.
/// Use for detector surfaces.
#[derive(Debug)]
pub struct Circle {
    /// Central position.
    centre: Point3<f64>,
    /// Normal.
    norm: Unit<Vector3<f64>>,
    /// Radius.
    rad: f64,
}

impl Circle {
    /// Construct a new instance.
    #[pre(rad > 0.0)]
    pub fn new(centre: Point3<f64>, norm: Unit<Vector3<f64>>, rad: f64) -> Self {
        Self { centre, norm, rad }
    }

    /// Reference the central position.
    pub fn centre(&self) -> &Point3<f64> {
        &self.centre
    }

    /// Reference the normal.
    pub fn norm(&self) -> &Unit<Vector3<f64>> {
        &self.norm
    }

    /// Get the radius.
    pub fn rad(&self) -> f64 {
        self.rad
    }

    /// Calculate the area.
    pub fn area(&self) -> f64 {
        PI * self.rad.powi(2)
    }
}

impl Traceable for Circle {
    fn intersect(&self, ray: &Ray) -> bool {
        let d = self.norm.dot(&ray.dir);

        if d <= EPSILON {
            return false;
        }

        let po = self.centre - ray.pos;
        let dist = po.dot(&self.norm) / d;
        let p = ray.pos + (ray.dir.into_inner() * dist);
        let v = p - self.centre;
        let d2 = v.dot(&v);

        d2 <= self.rad.powi(2)
    }

    fn dist(&self, ray: &Ray) -> Option<f64> {
        let d = self.norm.dot(&ray.dir);

        if d <= EPSILON {
            return None;
        }

        let pc = self.centre - ray.pos;
        let dist = pc.dot(&self.norm) / d;
        let p = ray.pos + (ray.dir.into_inner() * dist);
        let v = p - self.centre;
        let d2 = v.dot(&v);

        if d2 > self.rad.powi(2) {
            return None;
        }

        Some(dist)
    }

    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        let d = self.norm.dot(&ray.dir);

        if d <= EPSILON {
            return None;
        }

        let pc = self.centre - ray.pos;
        let dist = pc.dot(&self.norm) / d;
        let p = ray.pos + (ray.dir.into_inner() * dist);
        let v = p - self.centre;
        let d2 = v.dot(&v);

        if d2 > self.rad.powi(2) {
            return None;
        }

        Some((dist, self.norm))
    }
}
