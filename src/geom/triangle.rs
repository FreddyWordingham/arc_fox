//! Geometric triangle structure with interpolated phong normals.

use super::{Ray, Surface};
use nalgebra::{Point3, Unit, Vector3};

/// Parallel ray catch value.
const EPSILON: f64 = 1.0e-6;

/// Triangle in three-dimensions.
/// Commonly used to compose surfaces.
#[derive(Debug)]
pub struct Triangle {
    /// Normal vertex.
    normal: Unit<Vector3<f64>>,
    /// Vertex positions.
    verts: [Point3<f64>; 3],
    /// Vertex normal vectors.
    norms: [Unit<Vector3<f64>>; 3],
}

impl Triangle {
    /// Construct a new instance.
    pub fn new(verts: [Point3<f64>; 3], norms: [Unit<Vector3<f64>>; 3]) -> Self {
        Self {
            normal: Unit::new_normalize((verts[1] - verts[0]).cross(&(verts[2] - verts[0]))),
            verts,
            norms,
        }
    }

    /// Reference the normal vector.
    pub fn normal(&self) -> &Unit<Vector3<f64>> {
        &self.normal
    }

    /// Reference the vertex positions.
    pub fn verts(&self) -> &[Point3<f64>; 3] {
        &self.verts
    }

    /// Reference the vertex normals.
    pub fn norms(&self) -> &[Unit<Vector3<f64>>; 3] {
        &self.norms
    }

    /// Calculate the area of the triangle.
    pub fn area(&self) -> f64 {
        ((self.verts[1] - self.verts[0]).cross(&(self.verts[2] - self.verts[0]))).magnitude() / 2.0
    }
}

impl Surface for Triangle {
    fn intersect(&self, ray: &Ray) -> bool {
        let e01 = self.verts[1] - self.verts[0];
        let e02 = self.verts[2] - self.verts[0];

        let h = ray.direction().cross(&e02);
        let a = e01.dot(&h);

        if a.abs() < EPSILON {
            return false;
        }

        let f = 1.0 / a;
        let s = ray.origin() - self.verts[0];
        let u = f * s.dot(&h);

        if (u < 0.0) || (u > 1.0) {
            return false;
        }

        let q = s.cross(&e01);
        let v = f * ray.direction().dot(&q);

        if (v < 0.0) || ((u + v) > 1.0) {
            return false;
        }

        let dist = f * e02.dot(&q);

        if dist < EPSILON {
            return false;
        }

        true
    }

    fn distance(&self, ray: &Ray) -> Option<f64> {
        let e01 = self.verts[1] - self.verts[0];
        let e02 = self.verts[2] - self.verts[0];

        let h = ray.direction().cross(&e02);
        let a = e01.dot(&h);

        if a.abs() < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin() - self.verts[0];
        let u = f * s.dot(&h);

        if (u < 0.0) || (u > 1.0) {
            return None;
        }

        let q = s.cross(&e01);
        let v = f * ray.direction().dot(&q);

        if (v < 0.0) || ((u + v) > 1.0) {
            return None;
        }

        let dist = f * e02.dot(&q);

        if dist < EPSILON {
            return None;
        }

        Some(dist)
    }

    fn distance_normal(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        let e01 = self.verts()[1] - self.verts()[0];
        let e02 = self.verts()[2] - self.verts()[0];

        let h = ray.direction().cross(&e02);
        let a = e01.dot(&h);

        if a.abs() < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin() - self.verts()[0];
        let u = f * s.dot(&h);

        if (u < 0.0) || (u > 1.0) {
            return None;
        }

        let q = s.cross(&e01);
        let v = f * ray.direction().dot(&q);

        if (v < 0.0) || ((u + v) > 1.0) {
            return None;
        }

        let dist = f * e02.dot(&q);

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
