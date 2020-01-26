//! Verse form structure.

use crate::{ord::dom::Grid as DomGrid, sci::math::geom::shape::Aabb, uni::Verse};
use attr_mac::json;
use nalgebra::Point3;

/// Grid construction form.
#[json]
pub struct Grid {
    /// Grid resolution.
    res: [usize; 3],
    /// Min point.
    mins: Point3<f64>,
    /// Max point.
    maxs: Point3<f64>,
}

impl Grid {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 3], mins: Point3<f64>, maxs: Point3<f64>) -> Self {
        Self { res, mins, maxs }
    }

    // Form a manifested instance.
    #[inline]
    #[must_use]
    pub fn form(&self, verse: &Verse) -> DomGrid {
        let bound = Aabb::new(self.mins, self.maxs);

        DomGrid::new(bound, self.res, verse)
    }
}
