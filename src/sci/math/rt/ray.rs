//! Ray structure.

use contracts::{post, pre};
use nalgebra::{Point3, Unit, Vector3};

/// Ray structure implementation.
#[derive(Debug, Clone)]
pub struct Ray {
    /// Origin position.
    pos: Point3<f64>,
    /// Facing direction.
    dir: Unit<Vector3<f64>>,
}

impl Ray {
    /// Construct a new instance.
    #[pre((dir.magnitude_squared() - 1.0).abs() < 1.0e-6)]
    pub fn new(pos: Point3<f64>, dir: Unit<Vector3<f64>>) -> Self {
        Self { pos, dir }
    }

    /// Reference the origin position.
    pub fn pos(&self) -> &Point3<f64> {
        &self.pos
    }

    /// Reference the facing direction.
    #[post((ret.magnitude_squared() - 1.0).abs() < 1.0e-6)]
    pub fn dir(&self) -> &Unit<Vector3<f64>> {
        &self.dir
    }

    /// Move along the direction the given distance.
    #[pre(dist > 0.0)]
    pub fn travel(&mut self, dist: f64) {
        self.pos += self.dir.as_ref() * dist;
    }
}
