//! Geometric cube stucture.

use contracts::pre;
use nalgebra::Point3;

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
}
