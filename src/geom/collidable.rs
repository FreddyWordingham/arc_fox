//! Volume collidable trait.

use super::Aabb;

/// Types implementing this trait can be checked for aabb volume collisions.
pub trait Collidable {
    /// Determine if an collision occurs.
    fn collides(&self, cube: &Aabb) -> bool;
}
