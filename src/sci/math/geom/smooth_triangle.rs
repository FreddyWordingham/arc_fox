//! Smooth-triangle geometry structure.

use crate::{
    access,
    sci::math::geom::{Aabb, Collide, Triangle},
};
use nalgebra::{Unit, Vector3};

/// Triangle structure implementation with Phong normal interpolation.
/// Forms meshes.
pub struct SmoothTriangle {
    /// Base triangle.
    tri: Triangle,
    /// Normal vectors.
    norms: [Unit<Vector3<f64>>; 3],
}

impl SmoothTriangle {
    access!(tri, Triangle);
    access!(norms, [Unit<Vector3<f64>>; 3]);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(tri: Triangle, norms: [Unit<Vector3<f64>>; 3]) -> Self {
        if !norms.iter().all(|&n| n.dot(tri.plane_norm()) > 0.0) {
            panic!("Normals are not in direction with the plane.");
        }

        Self { tri, norms }
    }
}

impl Collide for SmoothTriangle {
    #[inline]
    #[must_use]
    fn bounding_box(&self) -> Aabb {
        self.tri.bounding_box()
    }

    #[inline]
    #[must_use]
    fn overlap(&self, aabb: &Aabb) -> bool {
        self.tri.overlap(aabb)
    }
}

