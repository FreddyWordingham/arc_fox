//! Binning functions.

use super::Resolution;
use crate::dom::{Aabb, Range};
use contracts::{post, pre};
use nalgebra::Point3;
use std::f64::MIN_POSITIVE;

/// Determine the index corresponding to the given value within the range.
#[pre(range.contains(x))]
#[post(ret < n)]
pub fn float(x: f64, range: &Range, n: usize) -> usize {
    (((x - range.min) / range.width()).min(1.0 - MIN_POSITIVE) * n as f64).floor() as usize
}

#[post(res.contains(&ret))]
pub fn point3(p: &Point3<f64>, aabb: &Aabb, res: &Resolution) -> [usize; 3] {
    [
        float(p.x, &Range::new(aabb.mins().x, aabb.maxs().x), res.x()),
        float(p.y, &Range::new(aabb.mins().y, aabb.maxs().y), res.y()),
        float(p.z, &Range::new(aabb.mins().z, aabb.maxs().z), res.z()),
    ]
}
