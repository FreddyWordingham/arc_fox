//! Binning functions.

use super::Layout3;
use crate::dom::{Aabb, Range};
use contracts::{post, pre};
use nalgebra::Point3;

/// Determine the index corresponding to the given value within the range.
#[pre(range.contains(x))]
#[post(ret < n)]
pub fn float(x: f64, range: &Range, n: usize) -> usize {
    (((x - range.min) / range.width()) * n as f64).floor() as usize
}

#[post(layout.contains(&ret))]
pub fn point3(p: &Point3<f64>, aabb: &Aabb, layout: &Layout3) -> [usize; 3] {
    [
        float(p.x, &Range::new(aabb.mins().x, aabb.maxs().x), layout.x()),
        float(p.y, &Range::new(aabb.mins().y, aabb.maxs().y), layout.y()),
        float(p.z, &Range::new(aabb.mins().z, aabb.maxs().z), layout.z()),
    ]
}
