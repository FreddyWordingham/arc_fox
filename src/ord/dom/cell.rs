//! Cell structure implementation.

use crate::sci::math::geom::Aabb;

/// Cell holding local information.
pub struct Cell {
    /// Boundary.
    bound: Aabb,
}

impl Cell {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(bound: Aabb) -> Self {
        Self { bound }
    }
}
