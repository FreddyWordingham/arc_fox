//! Geometric ray structure.

use nalgebra::{Point3, Unit, Vector3};

/// A line with an origin point that extends infinitely in one direction.
/// Commonly used to determine the distance to a geometric shape.
pub struct Ray {
    /// Origin of the line.
    origin: Point3<f64>,
    /// Direction of the line.
    direction: Unit<Vector3<f64>>,
}

impl Ray {
    /// Construct a new ray.
    pub fn new(origin: Point3<f64>, direction: Unit<Vector3<f64>>) -> Self {
        Self { origin, direction }
    }

    /// Reference the origin.
    pub fn origin(&self) -> &Point3<f64> {
        &self.origin
    }

    /// Reference the direction.
    pub fn direction(&self) -> &Unit<Vector3<f64>> {
        &self.direction
    }
}

/// Unit tests.
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let ray = Ray::new(
            Point3::new(1.0, -2.0, 3.14159),
            Unit::new_normalize(Vector3::new(1.0, -1.0, 0.0)),
        );

        assert_eq!(ray.origin(), &Point3::new(1.0, -2.0, 3.14159));
        assert_eq!(
            ray.direction(),
            &Unit::new_normalize(Vector3::new(1.0, -1.0, 0.0))
        );
    }
}
