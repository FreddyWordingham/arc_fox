//! Axis-aligned bounding box structure.

use super::EPSILON;
use crate::rt::{Ray, Traceable};
use contracts::{post, pre};
use nalgebra::{Point3, Unit, Vector3};

/// Axis-aligned box.
/// Used to partition domains.
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

    /// Construct a new instance centred on a point with given half_widths.
    #[pre(hws.x > 0.0)]
    #[pre(hws.y > 0.0)]
    #[pre(hws.z > 0.0)]
    pub fn new_centred(centre: &Point3<f64>, hws: &Vector3<f64>) -> Self {
        Self::new(centre - hws, centre + hws)
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
    #[post(ret.x > 0.0)]
    #[post(ret.y > 0.0)]
    #[post(ret.z > 0.0)]
    pub fn widths(&self) -> Vector3<f64> {
        self.maxs - self.mins
    }

    /// Calculate the half-widths.
    #[post(ret.x > 0.0)]
    #[post(ret.y > 0.0)]
    #[post(ret.z > 0.0)]
    pub fn half_widths(&self) -> Vector3<f64> {
        (self.maxs - self.mins) * 0.5
    }

    /// Calculate the centre position.
    pub fn centre(&self) -> Point3<f64> {
        nalgebra::center(&self.mins, &self.maxs)
    }

    /// Calculate the area.
    #[post(ret > 0.0)]
    pub fn area(&self) -> f64 {
        let ws = self.widths();

        2.0 * ((ws.x * ws.y) + (ws.y * ws.z) + (ws.z * ws.x))
    }

    /// Calculate the volume.
    #[post(ret > 0.0)]
    pub fn vol(&self) -> f64 {
        let ws = self.widths();

        ws.x * ws.y * ws.z
    }

    /// Determine the closest contained point to a given point.
    /// If the given point is contained, that value is returned.
    pub fn closest_point(&self, p: &Point3<f64>) -> Point3<f64> {
        let mut q = *p;

        for i in 0..3 {
            if p[i] < self.mins[i] {
                q[i] = self.mins[i];
            } else if p[i] > self.maxs[i] {
                q[i] = self.maxs[i];
            }
        }

        q
    }

    /// Determine the furthest contained point to a given point.
    /// If the given point is contained, the furthest point on the surface is returned.
    pub fn furthest_point(&self, p: &Point3<f64>) -> Point3<f64> {
        let mut q = *p;

        for i in 0..3 {
            if p[i] < self.mins[i] {
                q[i] = self.maxs[i];
            } else if p[i] > self.maxs[i] {
                q[i] = self.mins[i];
            } else {
                if (p[i] - self.mins[i]) < (self.maxs[i] - p[i]) {
                    q[i] = self.maxs[i];
                } else {
                    q[i] = self.mins[i];
                }
            }
        }

        q
    }
}

