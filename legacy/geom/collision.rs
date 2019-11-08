//! Collision trait.

use crate::geom::Aabb;

/// Geometry collision trait.
/// Structures implementing this trait can be tested for collision with an axis-aligned bounding box.
pub trait Collision {
    /// Check for a surface-volume overlap.
    fn overlap(&self, aabb: &Aabb) -> bool;
}
