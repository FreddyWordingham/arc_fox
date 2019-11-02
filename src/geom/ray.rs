//! Geometric ray structure.

use contracts::pre;
use nalgebra::{Point3, Unit, Vector3};
use serde::{Deserialize, Serialize};
use std::f64::consts::{FRAC_PI_2, PI};

/// Golden ratio: (1 + sqrt(5)) / 2.
const GOLDEN_RATIO: f64 = 1.61803398875;

#[pre(i.abs() <= n)]
/// Determine the ray casting direction for a given iteration of a Fibonacci spiral.
pub fn fibonacci_ray_cast(i: i32, n: i32) -> Unit<Vector3<f64>> {
    let theta = ((2.0 * i as f64) / ((2.0 * n as f64) + 1.0)).asin() + FRAC_PI_2;
    let phi = (i as f64 % GOLDEN_RATIO) * ((2.0 * PI) / GOLDEN_RATIO);

    Unit::new_normalize(Vector3::new(
        theta.sin() * phi.cos(),
        theta.sin() * phi.sin(),
        theta.cos(),
    ))
}

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
