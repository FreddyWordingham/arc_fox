//! Grid structure.

use super::Aabb;
use crate::index::Layout;

/// Domain cell grid.
pub struct Grid {
    /// Layout.
    layout: Layout,
    /// Boundary.
    aabb: Aabb,
}

impl Grid {
    /// Construct a new instance.
    pub fn new(layout: Layout, aabb: Aabb) -> Self {
        Self { layout, aabb }
    }

    /// Reference the boundary.
    pub fn aabb(&self) -> &Aabb {
        &self.aabb
    }
}
