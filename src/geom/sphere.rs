//! Geometic sphere structure.

use super::{Aabb, Collidable, Ray, Traceable};
use contracts::pre;
use nalgebra::{Isometry3, Point3, Unit, Vector3};

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
    pub fn rad() -> f64 {
        self.rad
    }
}
