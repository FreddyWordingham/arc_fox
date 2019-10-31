//! Geometric triangle structure with interpolated normals.

use super::{Aabb, Collidable, Ray, Traceable, Triangle};
use nalgebra::{Isometry3, Point3, Unit, Vector3};
use serde::{Deserialize, Serialize};

/// Parallel ray catch value.
const EPSILON: f64 = 1.0e-6;

/// Geometry describing a triangle in three-dimensional space.
/// The surface normal at a given point on the surface is interpolated using the Phong method.
/// Used to compose surfaces.
#[derive(Debug, Serialize, Deserialize)]
pub struct SmoothTriangle {
    /// Flat triangle surface.
    tri: Triangle,
    /// Vertex normals.
    norms: [Unit<Vector3<f64>>; 3],
}

impl SmoothTriangle {
    /// Construct a new instance.
    pub fn new(verts: [Point3<f64>; 3], norms: [Unit<Vector3<f64>>; 3]) -> Self {
        Self {
            tri: Triangle::new(verts),
            norms,
        }
    }

    /// Reference the vertex positions.
    pub fn verts(&self) -> &[Point3<f64>; 3] {
        &self.tri.verts()
    }

    /// Reference the surface normal.
    pub fn norm(&self) -> &Unit<Vector3<f64>> {
        &self.tri.norm()
    }

    /// Reference the vertex normals.
    pub fn norms(&self) -> &[Unit<Vector3<f64>>; 3] {
        &self.norms
    }

    /// Calculate the area.
    pub fn area(&self) -> f64 {
        self.tri.area()
    }

    /// Apply a transformation to the SmoothTriangle.
    pub fn transform(&mut self, trans: &Isometry3<f64>) {
        self.tri.transform(trans);

        for n in self.norms.iter_mut() {
            *n = trans * *n;
        }
    }
}

impl Traceable for SmoothTriangle {
    fn intersect(&self, ray: &Ray) -> bool {
        self.tri.intersect(ray)
    }

    fn dist(&self, ray: &Ray) -> Option<f64> {
        self.tri.dist(ray)
    }

    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        let verts = self.tri.verts();

        let e1 = verts[1] - verts[0];
        let e2 = verts[2] - verts[0];

        let h = ray.dir.cross(&e2);
        let a = e1.dot(&h);

        if a.abs() < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.pos - verts[0];
        let u = f * s.dot(&h);

        if (u < 0.0) || (u > 1.0) {
            return None;
        }

        let q = s.cross(&e1);
        let v = f * ray.dir.dot(&q);

        if (v < 0.0) || ((u + v) > 1.0) {
            return None;
        }

        let dist = f * e2.dot(&q);

        if dist < EPSILON {
            return None;
        }

        let w = 1.0 - u - v;

        Some((
            dist,
            Unit::new_normalize(
                (self.norms[1].into_inner() * u)
                    + (self.norms[2].into_inner() * v)
                    + (self.norms[0].into_inner() * w),
            ),
        ))
    }
}

impl Collidable for SmoothTriangle {
    fn collides(&self, aabb: &Aabb) -> bool {
        self.tri.collides(aabb)
    }

    fn boundary(&self) -> Aabb {
        self.tri.boundary()
    }
}
