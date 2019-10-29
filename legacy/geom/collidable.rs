//! Volume collidable trait.

use super::Cube;

/// Types implementing this trait can be checked for cube volume collided.
pub trait Collidable {
    /// Determine if an collision occurs at all.
    fn collides(&self, cube: &Cube) -> bool;
}
