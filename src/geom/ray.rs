//! Geometric ray structure.

use nalgebra::{Point3, Unit, Vector3};

/// Line with an origin point that extends infinitely in one direction.
/// Used to determine the distance to a geometric shape.
#[derive(Debug)]
pub struct Ray {
    /// Origin position.
    pos: Point3<f64>,
    /// Facing direction.
    dir: Unit<Vector3<f64>>,
}

impl Ray {
    /// Construct a new instance.
    pub fn new(pos: Point3<f64>, dir: Unit<Vector3<f64>>) -> Self {
        Self { pos, dir }
    }

    /// Reference the origin position.
    pub fn pos(&mut self) -> &mut Point3<f64> {
        &mut self.pos
    }

    /// Reference the facing direction.
    pub fn dir(&mut self) -> &mut Unit<Vector3<f64>> {
        &mut self.dir
    }
}
