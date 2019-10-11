//! Geometric cube stucture.

use contracts::pre;
use nalgebra::{Point3, Vector3};

/// An axis-aligned box.
/// Commonly used to partition the domain.
#[derive(Debug)]
pub struct Cube {
    /// Mininum bound.
    mins: Point3<f64>,
    /// Maximum bound.
    maxs: Point3<f64>,
}

impl Cube {
    /// Construct a new instance.
    #[pre(mins < maxs)]
    pub fn new(mins: Point3<f64>, maxs: Point3<f64>) -> Self {
        Self { mins, maxs }
    }

    /// Reference the minimum bound.
    pub fn mins(&self) -> &Point3<f64> {
        &self.mins
    }

    /// Reference the maximum bound.
    pub fn maxs(&self) -> &Point3<f64> {
        &self.maxs
    }

    /// Calculate the widths.
    pub fn widths(&self) -> Vector3<f64> {
        self.maxs - self.mins
    }

    /// Calculate the half-widths.
    pub fn half_widths(&self) -> Vector3<f64> {
        (self.maxs - self.mins) / 2.0
    }

    /// Calculate the centre position.
    pub fn centre(&self) -> Point3<f64> {
        nalgebra::center(&self.mins, &self.maxs)
    }

    /// Determine if the point is contained.
    /// Points lying exactly at the surface are considered contained.
    pub fn contained(&self, point: Point3<f64>) -> bool {
        (self.mins.x <= point.x)
            && (point.x <= self.maxs.x)
            && (self.mins.y <= point.y)
            && (point.y <= self.maxs.y)
            && (self.mins.z <= point.z)
            && (point.z <= self.maxs.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let cube = Cube::new(
            Point3::new(-1.0, 2.0, -3.14159),
            Point3::new(1.0, 4.0, 3.14159),
        );

        assert_eq!(cube.mins, Point3::new(-1.0, 2.0, -3.14159));
        assert_eq!(cube.maxs, Point3::new(1.0, 4.0, 3.14159));
    }

    #[test]
    fn getters() {
        let cube = Cube::new(
            Point3::new(-1.0, 2.0, -3.14159),
            Point3::new(1.0, 4.0, 3.14159),
        );

        assert_eq!(cube.mins(), &Point3::new(-1.0, 2.0, -3.14159));
        assert_eq!(cube.maxs(), &Point3::new(1.0, 4.0, 3.14159));
    }

    #[test]
    fn calculators() {
        let cube = Cube::new(
            Point3::new(-1.0, 2.0, -3.14159),
            Point3::new(1.0, 4.0, 3.14159),
        );

        assert_eq!(cube.widths(), Vector3::new(2.0, 2.0, 6.28318));
        assert_eq!(cube.half_widths(), Vector3::new(1.0, 1.0, 3.14159));
        assert_eq!(cube.centre(), Point3::new(0.0, 3.0, 0.0));
    }
}
