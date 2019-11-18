//! Aabb structure.

use super::super::Collide;
use crate::{
    base::{Index, Resolution},
    rt::{Ray, Trace},
    util::{Range, Tag},
};
use contracts::{post, pre};
use nalgebra::{Point3, Unit, Vector3};

/// Aabb structure implementation.
/// Quick first pass bounding volume.
#[derive(Debug, Clone)]
pub struct Aabb {
    /// Minimum bound.
    mins: Point3<f64>,
    /// Maximum bound.
    maxs: Point3<f64>,
}

impl Aabb {
    /// Construct a new instance.
    // #[pre(mins < maxs)] TODO: Supervise this.
    pub fn new(mins: Point3<f64>, maxs: Point3<f64>) -> Self {
        Self { mins, maxs }
    }

    /// Construct a new instance centred on a point with given half_widths.
    #[pre(hws.iter().all(|x| *x > 0.0))]
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
    #[post(ret.iter().all(|x| *x > 0.0))]
    pub fn widths(&self) -> Vector3<f64> {
        self.maxs - self.mins
    }

    /// Calculate the half-widths.
    #[post(ret.iter().all(|x| *x > 0.0))]
    pub fn half_widths(&self) -> Vector3<f64> {
        (self.maxs - self.mins) * 0.5
    }

    /// Calculate the centre position.
    pub fn centre(&self) -> Point3<f64> {
        nalgebra::center(&self.mins, &self.maxs)
    }

    /// Calculate the volume.
    #[post(ret > 0.0)]
    pub fn vol(&self) -> f64 {
        let ws = self.widths();
        ws.x * ws.y * ws.z
    }

    /// Create a tightened axis-aligned box.
    #[pre(f > 0.0)]
    #[pre(f < 1.0)]
    pub fn tighten(&self, f: f64) -> Self {
        let delta = self.half_widths() * f;
        Self::new(self.mins + delta, self.maxs - delta)
    }

    /// Create a loosened axis-aligned box.
    #[pre(f > 0.0)]
    pub fn loosen(&self, f: f64) -> Self {
        let delta = self.half_widths() * f;
        Self::new(self.mins - delta, self.maxs + delta)
    }

    /// Determine if the given point if contained.
    pub fn contains(&self, p: &Point3<f64>) -> bool {
        (p.x >= self.mins.x)
            && (p.x <= self.maxs.x)
            && (p.y >= self.mins.y)
            && (p.y <= self.maxs.y)
            && (p.z >= self.mins.z)
            && (p.z <= self.maxs.z)
    }

    /// Determine the index corresponding to a given point in the range.
    #[pre(self.contains(p))]
    #[post(res.contains(&ret))]
    pub fn find_index(&self, p: &Point3<f64>, res: &Resolution) -> Index {
        let xi = Range::new(self.mins.x, self.maxs.x).find_index(p.x, res.x());
        let yi = Range::new(self.mins.y, self.maxs.y).find_index(p.y, res.y());
        let zi = Range::new(self.mins.z, self.maxs.z).find_index(p.z, res.z());

        Index::new(xi, yi, zi)
    }
}

impl Collide for Aabb {
    fn bounding_box(&self) -> Aabb {
        self.clone()
    }

    fn overlap(&self, aabb: &Aabb) -> bool {
        (self.mins.x <= aabb.maxs.x)
            && (self.maxs.x >= aabb.mins.x)
            && (self.mins.y <= aabb.maxs.y)
            && (self.maxs.y >= aabb.mins.y)
            && (self.mins.z <= aabb.maxs.z)
            && (self.maxs.z >= aabb.mins.z)
    }
}

impl Trace for Aabb {
    fn hit(&self, ray: &Ray) -> bool {
        let dir_frac = Vector3::new(1.0 / ray.dir().x, 1.0 / ray.dir().y, 1.0 / ray.dir().z);

        let tx0 = (self.mins.x - ray.pos().x) * dir_frac.x;
        let tx1 = (self.maxs.x - ray.pos().x) * dir_frac.x;
        let ty0 = (self.mins.y - ray.pos().y) * dir_frac.y;
        let ty1 = (self.maxs.y - ray.pos().y) * dir_frac.y;
        let tz0 = (self.mins.z - ray.pos().z) * dir_frac.z;
        let tz1 = (self.maxs.z - ray.pos().z) * dir_frac.z;

        let t_min = (tx0.min(tx1)).max(ty0.min(ty1)).max(tz0.min(tz1));
        let t_max = (tx0.max(tx1)).min(ty0.max(ty1)).min(tz0.max(tz1));

        !((t_max < 0.0) || (t_min > t_max))
    }

    fn dist(&self, ray: &Ray) -> Option<f64> {
        let dir_frac = Vector3::new(1.0 / ray.dir().x, 1.0 / ray.dir().y, 1.0 / ray.dir().z);

        let tx0 = (self.mins.x - ray.pos().x) * dir_frac.x;
        let tx1 = (self.maxs.x - ray.pos().x) * dir_frac.x;
        let ty0 = (self.mins.y - ray.pos().y) * dir_frac.y;
        let ty1 = (self.maxs.y - ray.pos().y) * dir_frac.y;
        let tz0 = (self.mins.z - ray.pos().z) * dir_frac.z;
        let tz1 = (self.maxs.z - ray.pos().z) * dir_frac.z;

        let t_min = (tx0.min(tx1)).max(ty0.min(ty1)).max(tz0.min(tz1));
        let t_max = (tx0.max(tx1)).min(ty0.max(ty1)).min(tz0.max(tz1));

        if (t_max < 0.0) || (t_min > t_max) {
            return None;
        }

        if t_min > 0.0 {
            return Some(t_min);
        }

        Some(t_max)
    }

    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        let dir_frac = Vector3::new(1.0 / ray.dir().x, 1.0 / ray.dir().y, 1.0 / ray.dir().z);

        let t1 = Tag::new((self.mins.x - ray.pos().x) * dir_frac.x, -Vector3::x_axis());
        let t2 = Tag::new((self.maxs.x - ray.pos().x) * dir_frac.x, Vector3::x_axis());
        let t3 = Tag::new((self.mins.y - ray.pos().y) * dir_frac.y, -Vector3::y_axis());
        let t4 = Tag::new((self.maxs.y - ray.pos().y) * dir_frac.y, Vector3::y_axis());
        let t5 = Tag::new((self.mins.z - ray.pos().z) * dir_frac.z, -Vector3::z_axis());
        let t6 = Tag::new((self.maxs.z - ray.pos().z) * dir_frac.z, Vector3::z_axis());

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

    fn dist_inside(&self, ray: &Ray) -> Option<(f64, bool)> {
        if let Some(dist) = self.dist(ray) {
            return Some((dist, self.contains(&ray.pos())));
        }

        None
    }

    fn dist_inside_norm(&self, ray: &Ray) -> Option<(f64, bool, Unit<Vector3<f64>>)> {
        return if let Some((dist, norm)) = self.dist_norm(ray) {
            let inside = self.contains(&ray.pos());
            Some((dist, inside, norm))
        } else {
            None
        };
    }
}
