//! Axis-aligned bounding box structure.

use crate::sci::math::rt::Ray;
use nalgebra::{Point3, Vector3};
use std::cmp::Ordering;

/// Axis-aligned bounding box geometry.
/// Commonly used for spatial partitioning.
pub struct Aabb {
    /// Minimum bound.
    pub mins: Point3<f64>,
    /// Maximum bound.
    pub maxs: Point3<f64>,
}

impl Aabb {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(mins: Point3<f64>, maxs: Point3<f64>) -> Self {
        Self { mins, maxs }
    }

    /// Construct a new axis-aligned bounding box centred on a given point with given half widths.
    #[inline]
    #[must_use]
    pub fn new_centred(centre: &Point3<f64>, hws: &Vector3<f64>) -> Self {
        Self::new(centre - hws, centre + hws)
    }

    /// Calculate the widths.
    #[inline]
    #[must_use]
    pub fn widths(&self) -> Vector3<f64> {
        self.maxs - self.mins
    }

    /// Calculate the half-widths.
    #[inline]
    #[must_use]
    pub fn half_widths(&self) -> Vector3<f64> {
        self.widths() * 0.5
    }

    /// Calculate the centre position.
    #[inline]
    #[must_use]
    pub fn centre(&self) -> Point3<f64> {
        nalgebra::center(&self.mins, &self.maxs)
    }

    /// Calculate the volume.
    #[inline]
    #[must_use]
    pub fn vol(&self) -> f64 {
        let ws = self.widths();
        ws.x * ws.y * ws.z
    }

    /// Determine if the given point if contained.
    #[inline]
    #[must_use]
    pub fn contains(&self, p: &Point3<f64>) -> bool {
        p >= &self.mins && p <= &self.maxs
    }

    /// Determine the intersection distances along a ray's direction.
    #[must_use]
    fn intersections(&self, ray: &Ray) -> (f64, f64) {
        let t_0: Vec<_> = self
            .mins
            .iter()
            .zip(ray.pos.iter().zip(ray.dir.iter()))
            .map(|(m, (p, d))| (m - p) / d)
            .collect();

        let t_1: Vec<_> = self
            .maxs
            .iter()
            .zip(ray.pos.iter().zip(ray.dir.iter()))
            .map(|(m, (p, d))| (m - p) / d)
            .collect();

        let t_min = t_0
            .iter()
            .zip(t_1.iter())
            .map(|(a, b)| a.min(*b))
            .max_by(|a, b| {
                if a < b {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .expect("Invalid aabb bounds.");

        let t_max = t_0
            .iter()
            .zip(t_1.iter())
            .map(|(a, b)| a.max(*b))
            .min_by(|a, b| {
                if a < b {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .expect("Invalid aabb bounds.");

        (t_min, t_max)
    }
}
