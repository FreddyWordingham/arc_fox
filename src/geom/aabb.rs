//! Geometric cuboid structure.

use super::{Collidable, Ray, Shape, Traceable};
use crate::util::SortLabel;
use contracts::pre;
use nalgebra::{Point3, Unit, Vector3};
use serde::{Deserialize, Serialize};

/// Axis-aligned box.
/// Commonly used to partition domains.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

    /// Determine the closest contained point to a given point.
    pub fn closest_point(&self, p: &Point3<f64>) -> Point3<f64> {
        let mut q = Point3::origin();

        for i in 0..3 {
            if p[i] < self.mins[i] {
                q[i] = self.mins[i];
            } else if p[i] > self.maxs[i] {
                q[i] = self.maxs[i];
            }
        }

        q
    }
}

impl Traceable for Aabb {
    fn intersect(&self, ray: &Ray) -> bool {
        let dir_frac = Vector3::new(1.0 / ray.dir.x, 1.0 / ray.dir.y, 1.0 / ray.dir.z);

        let tx0 = (self.mins.x - ray.pos.x) * dir_frac.x;
        let tx1 = (self.maxs.x - ray.pos.x) * dir_frac.x;
        let ty0 = (self.mins.y - ray.pos.y) * dir_frac.y;
        let ty1 = (self.maxs.y - ray.pos.y) * dir_frac.y;
        let tz0 = (self.mins.z - ray.pos.z) * dir_frac.z;
        let tz1 = (self.maxs.z - ray.pos.z) * dir_frac.z;

        let t_min = (tx0.min(tx1)).max(ty0.min(ty1)).max(tz0.min(tz1));
        let t_max = (tx0.max(tx1)).min(ty0.max(ty1)).min(tz0.max(tz1));

        !((t_max < 0.0) || (t_min > t_max))
    }

    fn dist(&self, ray: &Ray) -> Option<f64> {
        let dir_frac = Vector3::new(1.0 / ray.dir.x, 1.0 / ray.dir.y, 1.0 / ray.dir.z);

        let t1 = (self.mins.x - ray.pos.x) * dir_frac.x;
        let t2 = (self.maxs.x - ray.pos.x) * dir_frac.x;
        let t3 = (self.mins.y - ray.pos.y) * dir_frac.y;
        let t4 = (self.maxs.y - ray.pos.y) * dir_frac.y;
        let t5 = (self.mins.z - ray.pos.z) * dir_frac.z;
        let t6 = (self.maxs.z - ray.pos.z) * dir_frac.z;

        let t_min = (t1.min(t2)).max(t3.min(t4)).max(t5.min(t6));
        let t_max = (t1.max(t2)).min(t3.max(t4)).min(t5.max(t6));

        if (t_max < 0.0) || (t_min > t_max) {
            return None;
        }

        if t_min > 0.0 {
            return Some(t_min);
        }

        Some(t_max)
    }

    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        let dir_frac = Vector3::new(1.0 / ray.dir.x, 1.0 / ray.dir.y, 1.0 / ray.dir.z);

        let t1 = SortLabel::new((self.mins.x - ray.pos.x) * dir_frac.x, -Vector3::x_axis());
        let t2 = SortLabel::new((self.maxs.x - ray.pos.x) * dir_frac.x, Vector3::x_axis());
        let t3 = SortLabel::new((self.mins.y - ray.pos.y) * dir_frac.y, -Vector3::y_axis());
        let t4 = SortLabel::new((self.maxs.y - ray.pos.y) * dir_frac.y, Vector3::y_axis());
        let t5 = SortLabel::new((self.mins.z - ray.pos.z) * dir_frac.z, -Vector3::z_axis());
        let t6 = SortLabel::new((self.maxs.z - ray.pos.z) * dir_frac.z, Vector3::z_axis());

        let t_min = (&t1.min(&t2)).max(&t3.min(&t4)).max(&t5.min(&t6));
        let t_max = (&t1.max(&t2)).min(&t3.max(&t4)).min(&t5.max(&t6));

        if (t_max < 0.0) || (t_min > t_max) {
            return None;
        }

        if t_min > 0.0 {
            return Some(t_min.components());
        }

        Some(t_max.components())
    }
}

impl Collidable for Aabb {
    fn collides(&self, aabb: &Aabb) -> bool {
        (self.mins.x <= aabb.maxs.x)
            && (self.maxs.x >= aabb.mins.x)
            && (self.mins.y <= aabb.maxs.y)
            && (self.maxs.y >= aabb.mins.y)
            && (self.mins.z <= aabb.maxs.z)
            && (self.maxs.z >= aabb.mins.z)
    }

    fn boundary(&self) -> Aabb {
        self.clone()
    }
}

impl Shape for Aabb {}
