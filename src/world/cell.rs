//! Domain cell structure.

use crate::geom::Aabb;

/// Domain cell structure.
/// Contains local spatial information.
#[derive(Debug)]
pub struct Cell {
    /// Boundary.
    boundary: Aabb,
}

impl Cell {
    /// Construct a new instance.
    pub fn new(boundary: Aabb) -> Self {
        Self { boundary }
    }
}
