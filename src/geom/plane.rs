//! Plane structure.

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
