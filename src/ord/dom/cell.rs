//! Cell structure implementation.

use crate::{access, sci::math::geom::Aabb};

/// Cell holding local information.
pub struct Cell {
    /// Boundary.
    bound: Aabb,
}

impl Cell {
    access!(bound, Aabb);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(bound: Aabb) -> Self {
        Self { bound }
    }
}
