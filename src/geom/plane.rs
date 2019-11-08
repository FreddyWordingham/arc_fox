//! Axis-aligned bounding box structure.

use super::Touchable;
use nalgebra::{Point3, Unit, Vector3};

/// Plane geometry.
pub struct Plane {
    /// Point on the plane.
    pos: Point3<f64>,
    /// Normal of the plane.
    norm: Unit<Vector3<f64>>,
}

impl Plane {
    /// Construct a new instance.
    pub fn new(pos: Point3<f64>, norm: Unit<Vector3<f64>>) -> Self {
        Self { pos, norm }
    }
}

impl Touchable for Plane {
    fn closest_point(&self, p: &Point3<f64>) -> Point3<f64> {
        let t = (self.norm.dot(&p.coords) - (self.pos.coords.dot(&self.norm)))
            / self.norm.dot(&self.norm);

        p - (self.norm.as_ref() * t)
    }
}
