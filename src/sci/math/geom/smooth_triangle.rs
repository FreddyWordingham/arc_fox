//! Smooth-triangle geometry structure.

use crate::{
    access,
    sci::math::{
        geom::{Aabb, Collide, Triangle},
        rt::{Ray, Trace},
    },
    util::list::alphabet::Greek::{Alpha, Beta, Gamma},
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

impl Trace for SmoothTriangle {
    #[inline]
    #[must_use]
    fn hit(&self, ray: &Ray) -> bool {
        self.tri.intersection_coors(ray).is_some()
    }

    #[inline]
    #[must_use]
    fn dist(&self, ray: &Ray) -> Option<f64> {
        if let Some((dist, _coors)) = self.tri.intersection_coors(ray) {
            return Some(dist);
        }

        None
    }

    #[inline]
    #[must_use]
    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        if let Some((dist, [u, v, w])) = self.tri.intersection_coors(ray) {
            return Some((
                dist,
                Unit::new_normalize(
                    (self.norms[Beta as usize].into_inner() * u)
                        + (self.norms[Gamma as usize].into_inner() * v)
                        + (self.norms[Alpha as usize].into_inner() * w),
                ),
            ));
        }

        None
    }

    #[inline]
    #[must_use]
    fn dist_inside(&self, ray: &Ray) -> Option<(f64, bool)> {
        if let Some(dist) = self.dist(ray) {
            Some((dist, self.tri.plane_norm().dot(ray.dir()) > 0.0))
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    fn dist_inside_norm(&self, ray: &Ray) -> Option<(f64, bool, Unit<Vector3<f64>>)> {
        if let Some((dist, norm)) = self.dist_norm(ray) {
            let inside = ray.dir().dot(self.tri.plane_norm()) > 0.0;
            Some((dist, inside, norm))
        } else {
            None
        }
    }
}
