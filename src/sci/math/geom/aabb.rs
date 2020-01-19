//! Axis-aligned bounding box geometry structure.

use crate::{
    access,
    sci::math::{
        geom::Collide,
        rt::{Ray, Trace},
    },
};
use nalgebra::{Point3, Unit, Vector3};
use std::cmp::Ordering;

/// Axis-aligned bounding box geometry.
/// Commonly used for spatial partitioning.
#[derive(Clone)]
pub struct Aabb {
    /// Minimum bound.
    mins: Point3<f64>,
    /// Maximum bound.
    maxs: Point3<f64>,
}

impl Aabb {
    access!(mins, Point3<f64>);
    access!(maxs, Point3<f64>);

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

    /// Calculate the surface area.
    #[inline]
    #[must_use]
    pub fn area(&self) -> f64 {
        let ws = self.widths();
        2.0 * ws.z.mul_add(ws.x, ws.x.mul_add(ws.y, ws.y * ws.z))
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

    /// Calculate the distance to the aabb from a given point.
    #[inline]
    #[must_use]
    pub fn dist_sq_from_point(&self, p: &Point3<f64>) -> f64 {
        let mut dist_sq = 0.0;

        for (v, (min, max)) in p.iter().zip(self.mins.iter().zip(self.maxs.iter())) {
            if v < min {
                dist_sq += (min - v).powi(2);
            } else if v > max {
                dist_sq += (v - max).powi(2);
            }
        }

        dist_sq
    }

    /// Determine the intersection distances along a ray's direction.
    #[must_use]
    fn intersections(&self, ray: &Ray) -> (f64, f64) {
        let t_0: Vec<_> = self
            .mins
            .iter()
            .zip(ray.pos().iter().zip(ray.dir().iter()))
            .map(|(m, (p, d))| (m - p) / d)
            .collect();

        let t_1: Vec<_> = self
            .maxs
            .iter()
            .zip(ray.pos().iter().zip(ray.dir().iter()))
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

impl Collide for Aabb {
    #[inline]
    #[must_use]
    fn bounding_box(&self) -> Aabb {
        self.clone()
    }

    #[inline]
    #[must_use]
    fn overlap(&self, aabb: &Aabb) -> bool {
        self.mins <= aabb.maxs && self.maxs >= aabb.mins
    }
}

impl Trace for Aabb {
    #[inline]
    #[must_use]
    fn hit(&self, ray: &Ray) -> bool {
        let (t_min, t_max) = self.intersections(ray);

        !(t_max <= 0.0 || t_min > t_max)
    }

    #[inline]
    #[must_use]
    fn dist(&self, ray: &Ray) -> Option<f64> {
        let (t_min, t_max) = self.intersections(ray);

        if t_max <= 0.0 || t_min > t_max {
            return None;
        }

        if t_min > 0.0 {
            return Some(t_min);
        }

        Some(t_max)
    }

    #[inline]
    #[must_use]
    fn dist_norm(&self, _ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        unimplemented!("Tell me (Freddy) if you need this.");
    }

    #[inline]
    #[must_use]
    fn dist_inside(&self, ray: &Ray) -> Option<(f64, bool)> {
        if let Some(dist) = self.dist(ray) {
            return Some((dist, self.contains(ray.pos())));
        }

        None
    }

    #[inline]
    #[must_use]
    fn dist_inside_norm(&self, ray: &Ray) -> Option<(f64, bool, Unit<Vector3<f64>>)> {
        if let Some((dist, norm)) = self.dist_norm(ray) {
            let inside = self.contains(ray.pos());
            Some((dist, inside, norm))
        } else {
            None
        }
    }
}
