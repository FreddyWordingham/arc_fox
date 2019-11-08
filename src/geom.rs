//! Geometric shape enumeration.

pub mod aabb;
pub mod collision;
pub mod container;
pub mod mesh;
pub mod plane;
pub mod touchable;
pub mod triangle;

pub use self::aabb::*;
pub use self::collision::*;
pub use self::container::*;
pub use self::mesh::*;
pub use self::plane::*;
pub use self::touchable::*;
pub use self::triangle::*;

/// Parallel ray catch value.
const EPSILON: f64 = 1.0e-6;
