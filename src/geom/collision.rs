//! Collision trait.

use crate::geom::Aabb;

/// Geometry collision trait.
/// Structures implementing this trait can be tested for collision with an axis-aligned bounding box.
pub trait Collision {
    /// Construct an axis-aligned bounding box for the geometry.
    fn bounding_box(&self) -> Aabb;

    /// Check for an overlapping collision.
    fn overlap(&self, aabb: &Aabb) -> bool;
}
