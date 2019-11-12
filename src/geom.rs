//! Geometric structures and traits.

pub mod aabb;
pub mod collision;
pub mod mesh;
pub mod transform;
pub mod triangle;

pub use self::aabb::*;
pub use self::collision::*;
pub use self::mesh::*;
pub use self::transform::*;
pub use self::triangle::*;

/// Parallel ray catch value.
const EPSILON: f64 = 1.0e-6;
