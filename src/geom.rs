//! Geometric shape enumeration.

pub mod aabb;
pub mod collision;
pub mod container;

pub use self::aabb::*;
pub use self::collision::*;
pub use self::container::*;

/// Parallel ray catch value.
const EPSILON: f64 = 1.0e-6;
