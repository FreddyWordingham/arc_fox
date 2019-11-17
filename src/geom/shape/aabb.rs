//! Aabb structure.

use crate::{
    base::{Index, Resolution},
    util::Range,
};
use contracts::{post, pre};
use nalgebra::{Point3, Vector3};

/// Aabb structure implementation.
/// Quick first pass bounding volume.
#[derive(Debug)]
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
