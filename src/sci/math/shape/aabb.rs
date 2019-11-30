//! Axis-aligned bounding box structure.

use super::super::{
    geom::Collide,
    rt::{Ray, Trace},
    Normal,
};
use contracts::{post, pre};
use nalgebra::{Point3, Unit, Vector3};
use std::cmp::Ordering;

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
    #[pre(mins < maxs)]
    pub fn new(mins: Point3<f64>, maxs: Point3<f64>) -> Self {
        Self { mins, maxs }
    }

    /// Construct a new instance centred on a point with given half_widths.
    #[pre(hws.iter().all(|hw| *hw > 0.0))]
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
    #[post(ret.iter().all(|w| *w > 0.0))]
    pub fn widths(&self) -> Vector3<f64> {
        self.maxs - self.mins
    }

    /// Calculate the half-widths.
    #[post(ret.iter().all(|hw| *hw > 0.0))]
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
        p >= &self.mins && p <= &self.maxs
    }

    /// Determine the intersection distances along a ray's direction.
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
            .unwrap();

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
            .unwrap();

        (t_min, t_max)
    }
}

impl Collide for Aabb {
    fn bounding_box(&self) -> Aabb {
        self.clone()
    }

    fn overlap(&self, aabb: &Aabb) -> bool {
        self.mins <= aabb.maxs && self.maxs >= aabb.mins
    }
}

impl Trace for Aabb {
    fn hit(&self, ray: &Ray) -> bool {
        let (t_min, t_max) = self.intersections(&ray);

        !(t_max <= 0.0 || t_min > t_max)
    }

    #[post(ret.is_none() || ret.unwrap() > 0.0)]
    fn dist(&self, ray: &Ray) -> Option<f64> {
        let (t_min, t_max) = self.intersections(&ray);

        if t_max <= 0.0 || t_min > t_max {
            return None;
        }

        if t_min > 0.0 {
            return Some(t_min);
        }

        Some(t_max)
    }

    #[post(ret.is_none() || (ret.unwrap().0 > 0.0 && (ret.unwrap().1.is_normal())))]
    fn dist_norm(&self, _ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        unimplemented!("Tell me if you need this.");
    }

    #[post(ret.is_none() || ret.unwrap().0 > 0.0)]
    fn dist_inside(&self, ray: &Ray) -> Option<(f64, bool)> {
        if let Some(dist) = self.dist(ray) {
            return Some((dist, self.contains(&ray.pos())));
        }

        None
    }

    #[post(ret.is_none() || (ret.unwrap().0 > 0.0 && (ret.unwrap().2.is_normal())))]
    fn dist_inside_norm(&self, ray: &Ray) -> Option<(f64, bool, Unit<Vector3<f64>>)> {
        return if let Some((dist, norm)) = self.dist_norm(ray) {
            let inside = self.contains(&ray.pos());
            Some((dist, inside, norm))
        } else {
            None
        };
    }
}
