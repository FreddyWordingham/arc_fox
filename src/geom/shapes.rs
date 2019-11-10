//! Geometric shape structures.

pub mod aabb;
pub mod mesh;
pub mod triangle;

pub use self::aabb::*;
pub use self::mesh::*;
pub use self::triangle::*;

/// Parallel ray catch value.
const EPSILON: f64 = 1.0e-6;
