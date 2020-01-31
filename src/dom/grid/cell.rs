//! Cell structure implementation.

use crate::{access, dom::Name, geom::Aabb};

/// Cell holding local information.
pub struct Cell {
    /// Boundary.
    bound: Aabb,
    /// Central material.
    mat: Name,
    // /// Intersecting interface triangles.
    // inter_tris: Vec<(&'a (Name, Interface), Vec<&'a SmoothTriangle>)>,
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
