//! Cell structure.

use super::Aabb;

/// Single domain cell.
pub struct Cell {
    /// Boundary.
    aabb: Aabb,
}

impl Cell {
    /// Construct a new instance.
    pub fn new(aabb: Aabb) -> Self {
        Self { aabb }
    }

    /// Reference the boundary.
    pub fn aabb(&self) -> &Aabb {
        &self.aabb
    }
}
