//! Geometric shape enumeration.

pub mod aabb;
pub mod collision;
pub mod plane;
pub mod shape;

pub use self::aabb::*;
pub use self::collision::*;
pub use self::plane::*;
pub use self::shape::*;

/// Parallel ray catch value.
const EPSILON: f64 = 1.0e-6;
