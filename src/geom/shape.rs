//! shape trait.

use super::{Collidable, Traceable};

/// Types implementing this trait can be traced using a ray and be checked for aabb volume collisions.
pub trait Shape: Traceable + Collidable {}
