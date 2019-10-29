//! Geometric plane structure.

use nalgebra::{Point3, Unit, Vector3};

/// Infinite plane geometry.
#[derive(Debug)]
pub struct Plane {
    /// Position on the plane.
    pos: Point3<f64>,
    /// Normal of the plane.
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
