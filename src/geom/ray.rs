//! Geometric ray structure.

use contracts::pre;
use nalgebra::{Point3, Rotation3, Unit, Vector3};

/// A line with an origin point that extends infinitely in one direction.
/// Commonly used to determine the distance to a geometric shape.
#[derive(Debug)]
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

    /// Move along the direction the given distance.
    #[pre(dist > 0.0)]
    pub fn travel(&mut self, dist: f64) {
        self.origin += self.direction.as_ref() * dist;
    }

    /// Pitch towards the z-axis and then roll around previous direction.
    #[pre(self.direction.z != 1.0)]
    pub fn rotate(&mut self, pitch: f64, roll: f64) {
        let pitch_axis = Unit::new_unchecked(self.direction.cross(&Vector3::z_axis()));
        let pitch_rot = Rotation3::from_axis_angle(&pitch_axis, pitch);

        let roll_rot = Rotation3::from_axis_angle(&self.direction, roll);

        self.direction = roll_rot * pitch_rot * self.direction;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use std::f64::consts::{FRAC_PI_4, FRAC_PI_8};

    #[test]
    fn new() {
        let ray = Ray::new(
            Point3::new(1.0, -2.0, 3.14159),
            Unit::new_normalize(Vector3::new(1.0, -1.0, 0.0)),
        );

        assert_eq!(ray.origin, Point3::new(1.0, -2.0, 3.14159));
        assert_eq!(
            ray.direction,
            Unit::new_normalize(Vector3::new(1.0, -1.0, 0.0))
        );
    }

    #[test]
    fn getters() {
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

    #[test]
    fn rotation() {
        let mut ray = Ray::new(Point3::new(1.0, -2.0, 3.14159), Vector3::x_axis());
        ray.rotate(FRAC_PI_8, FRAC_PI_4);

        assert_approx_eq!(ray.direction.x, FRAC_PI_8.cos());
        assert_approx_eq!(ray.direction.y, FRAC_PI_8.sin() * -FRAC_PI_4.cos());
        assert_approx_eq!(ray.direction.z, FRAC_PI_8.sin() * FRAC_PI_4.sin());
    }

    #[test]
    fn rotate() {
        let mut ray = Ray::new(Point3::new(1.0, -2.0, 3.14159), Vector3::x_axis());
        ray.rotate(FRAC_PI_8, FRAC_PI_4);

        assert_approx_eq!(ray.direction.x, FRAC_PI_8.cos());
        assert_approx_eq!(ray.direction.y, FRAC_PI_8.sin() * -FRAC_PI_4.cos());
        assert_approx_eq!(ray.direction.z, FRAC_PI_8.sin() * FRAC_PI_4.sin());
    }
}
