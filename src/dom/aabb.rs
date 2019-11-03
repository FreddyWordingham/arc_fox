//! Geometric cuboid structure.

use crate::geom::Shape;
use contracts::{post, pre};
use nalgebra::{Point3, Vector3};

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

    /// Determine if the given shape's surface intersects the aabb's surface.
    pub fn intersect(&self, shape: &Shape) -> bool {
        return match shape {
            Shape::Plane { dist, norm } => {
                let c = self.centre();
                let e = self.half_widths();

                let r = (e.x * norm.x.abs()) + (e.y * norm.y.abs()) + (e.z * norm.z.abs());
                let s = norm.dot(&c.coords) - dist;

                s.abs() <= r
            }
        };
    }

    /// Determine if the given shape's volume overlaps with the aabb's.
    pub fn overlap(&self, shape: &Shape) -> bool {
        return match shape {
            Shape::Plane { dist, norm } => {
                let c = self.centre();
                let e = self.half_widths();

                let r = (e.x * norm.x.abs()) + (e.y * norm.y.abs()) + (e.z * norm.z.abs());
                let s = norm.dot(&c.coords) - dist;

                s.abs() <= r
            }
        };
    }
}
