//! Collide trait.

use super::shape::Aabb;
use std::fmt::Debug;

/// Collide trait implementation.
/// Structures implementing this trait can be tested for collision with an axis-aligned bounding box.
pub trait Collide: Debug {
    /// Construct an axis-aligned bounding box for the geometry.
    fn bounding_box(&self) -> Aabb;

    /// Check for an overlapping collision.
    fn overlap(&self, aabb: &Aabb) -> bool;
}
