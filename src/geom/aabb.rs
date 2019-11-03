//! Geometric cuboid structure.

use contracts::pre;
use nalgebra::{Point3, Vector3};

/// Axis-aligned box.
/// Used to partition domains.
#[derive(Debug, Clone)]
pub struct Aabb {
    /// Minimum bound.
    mins: Point3<f64>,
    /// Maximum bound.
    maxs: Point3<f64>,
}

impl Aabb {
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

    /// Calculate the area.
    pub fn area(&self) -> f64 {
        let ws = self.widths();

        2.0 * ((ws.x * ws.y) + (ws.y * ws.z) + (ws.z * ws.x))
    }

    /// Calculate the volume.
    pub fn vol(&self) -> f64 {
        let ws = self.widths();

        ws.x * ws.y * ws.z
    }

    /// Determine if the given point is contained.
    /// Points lying exactly at the surface are considered contained.
    pub fn contains(&self, point: &Point3<f64>) -> bool {
        (self.mins.x <= point.x)
            && (point.x <= self.maxs.x)
            && (self.mins.y <= point.y)
            && (point.y <= self.maxs.y)
            && (self.mins.z <= point.z)
            && (point.z <= self.maxs.z)
    }
}
