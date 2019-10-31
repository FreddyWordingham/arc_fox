//! Geometric ray structure.

use nalgebra::{Point3, Unit, Vector3};
use serde::{Deserialize, Serialize};

/// Line with an origin point that extends infinitely in one direction.
/// Used to determine the distance to a geometric shape.
#[derive(Debug, Serialize, Deserialize)]
pub struct Ray {
    /// Origin position.
    pub pos: Point3<f64>,
    /// Facing direction.
    pub dir: Unit<Vector3<f64>>,
}

impl Ray {
    /// Construct a new instance.
    pub fn new(pos: Point3<f64>, dir: Unit<Vector3<f64>>) -> Self {
        Self { pos, dir }
    }
}
