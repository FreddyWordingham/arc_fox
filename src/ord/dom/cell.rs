//! Cell structure implementation.

use crate::{access, ord::Name, sci::math::geom::shape::Aabb};

/// Cell holding local information.
pub struct Cell {
    /// Boundary.
    bound: Aabb,
    /// Central material.
    mat: Name,
}

impl Cell {
    access!(bound, Aabb);
    access!(mat, Name);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(bound: Aabb, mat: Name) -> Self {
        Self { bound, mat }
    }
}
