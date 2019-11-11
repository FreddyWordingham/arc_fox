//! Geometric ray structure.

use contracts::pre;
use nalgebra::{Point3, Rotation3, Unit, Vector3};
use std::f64::consts::{FRAC_PI_2, PI};

/// Solution to the quadratic equation x^2 - x - 1 = 0.
const GOLDEN_RATIO: f64 = 1.61803398875;

/// Line with an origin point that extends infinitely in one direction.
/// Used to determine the distance to a geometric shape.
#[derive(Clone)]
pub struct Ray {
    /// Origin position.
    pub pos: Point3<f64>,
    /// Facing direction.
    pub dir: Unit<Vector3<f64>>,
}

impl Ray {
    /// Construct a new instance.
    #[pre((dir.magnitude() - 1.0).abs() < 1.0e-9)]
    pub fn new(pos: Point3<f64>, dir: Unit<Vector3<f64>>) -> Self {
        Self { pos, dir }
    }

    /// Determine the ray casting direction for a given iteration of a Fibonacci spiral.
    #[pre(i.abs() <= n)]
    pub fn new_fibonacci_spiral(p: Point3<f64>, i: i32, n: i32) -> Self {
        let theta = ((2.0 * i as f64) / ((2.0 * n as f64) + 1.0)).asin() + FRAC_PI_2;
        let phi = (i as f64 % GOLDEN_RATIO) * ((2.0 * PI) / GOLDEN_RATIO);

        Self::new(
            p,
            Unit::new_normalize(Vector3::new(
                theta.sin() * phi.cos(),
                theta.sin() * phi.sin(),
                theta.cos(),
            )),
        )
    }

    /// Move along the direction the given distance.
    #[pre(dist > 0.0)]
    pub fn travel(&mut self, dist: f64) {
        self.pos += self.dir.as_ref() * dist;
    }

    /// Pitch towards the z-axis and then roll around previous direction.
    #[pre(self.dir.z.abs() != 1.0)]
    #[post(self.dir.magnitude() == 1.0)]
    pub fn rotate(&mut self, pitch: f64, roll: f64) {
        let pitch_axis = Unit::new_normalize(self.dir.cross(&Vector3::z_axis()));
        let pitch_rot = Rotation3::from_axis_angle(&pitch_axis, pitch);

        let roll_rot = Rotation3::from_axis_angle(&self.dir, roll);

        self.dir = roll_rot * pitch_rot * self.dir;
    }
}
