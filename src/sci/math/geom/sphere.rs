//! Sphere geometry structure.

use nalgebra::Point3;
use std::f64::consts::PI;

/// Sphere geometry.
#[derive(Clone)]
pub struct Sphere {
    /// Central point.
    pub pos: Point3<f64>,
    /// Radius.
    pub rad: f64,
}

impl Sphere {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(pos: Point3<f64>, rad: f64) -> Self {
        Self { pos, rad }
    }

    /// Calculate the surface area.
    #[inline]
    #[must_use]
    pub fn area(&self) -> f64 {
        4.0 * PI * self.rad.powi(2)
    }

    /// Calculate the volume.
    #[inline]
    #[must_use]
    pub fn vol(&self) -> f64 {
        4.0 / 3.0 * PI * self.rad.powi(3)
    }

    /// Determine if the given point if contained.
    #[inline]
    #[must_use]
    pub fn contains(&self, p: &Point3<f64>) -> bool {
        nalgebra::distance_squared(&self.pos, p) <= self.rad.powi(2)
    }
}
