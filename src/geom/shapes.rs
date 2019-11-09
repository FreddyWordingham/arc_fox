//! Geometric shape structures.

pub mod aabb;

pub use self::aabb::*;

/// Parallel ray catch value.
const EPSILON: f64 = 1.0e-6;
