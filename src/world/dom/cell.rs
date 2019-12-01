//! Cell structure.

use crate::sci::math::shape::Aabb;

/// Cell structure implementation.
#[derive(Debug)]
pub struct Cell {
    /// Boundary.
    boundary: Aabb,
    // /// Intersecting interface triangles.
    // inter_tris: Vec<(&'a Interface<'a>, Vec<&'a Triangle>)>,
    // /// Central material.
    // mat: &'a Material,
}

impl Cell {
    /// Construct a new instance.
    pub fn new(boundary: Aabb) -> Self {
        Self { boundary }
    }
}
