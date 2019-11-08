//! Geometric shape enumeration.

pub mod aabb;
pub mod collision;
pub mod container;
pub mod plane;
pub mod touchable;

pub use self::aabb::*;
pub use self::collision::*;
pub use self::container::*;
pub use self::plane::*;
pub use self::touchable::*;

/// Parallel ray catch value.
const EPSILON: f64 = 1.0e-6;
