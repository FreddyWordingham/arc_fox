//! Axis-aligned bounding box structure.

use super::super::geom::Collide;
use contracts::{post, pre};
use nalgebra::{Point3, Vector3};

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
}

impl Collide for Aabb {
    fn bounding_box(&self) -> Aabb {
        self.clone()
    }

    fn overlap(&self, aabb: &Aabb) -> bool {
        self.mins <= aabb.maxs && self.maxs >= aabb.mins
    }
}
