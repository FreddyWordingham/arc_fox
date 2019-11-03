//! Shape enumeration.

use contracts::pre;
use nalgebra::{Unit, Vector3};

/// Shape enumeration.
/// Used to compose entities.
pub enum Shape {
    /// Plane shape.
    Plane {
        /// Distance from the origin to the plane.
        dist: f64,
        /// Normal.
        norm: Unit<Vector3<f64>>,
    },
}

impl Shape {
    /// Construct a new Plane instance.
    #[pre(dist >= 0.0)]
    pub fn new_plane(dist: f64, norm: Unit<Vector3<f64>>) -> Self {
        Self::Plane { dist, norm }
    }
}
