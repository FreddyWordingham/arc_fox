//! Triangle structure.

// use super::Collision;
use super::EPSILON;
use crate::rt::{Ray, Traceable};
use nalgebra::{Point3, Unit, Vector3};

/// Triangle geometry with normal interpolation.
pub struct Triangle {
    /// Vertex points.
    verts: [Point3<f64>; 3],
    /// Normal vectors.
    norms: [Unit<Vector3<f64>>; 3],
    /// Surface plane normal.
    plane_norm: Unit<Vector3<f64>>,
}

impl Triangle {
    /// Construct a new object.
    pub fn new(verts: [Point3<f64>; 3], norms: [Unit<Vector3<f64>>; 3]) -> Self {
        let plane_norm = Unit::new_normalize((verts[1] - verts[0]).cross(&(verts[2] - verts[0])));

        if !norms.iter().all(|&n| n.dot(&plane_norm) > 0.0) {
            panic!("Normals are not in direction with the plane!");
        }

        Self {
            verts,
            norms,
            plane_norm,
        }
    }
}

impl Traceable for Triangle {
    fn hit(&self, ray: &Ray) -> bool {
        let e1 = self.verts[1] - self.verts[0];
        let e2 = self.verts[2] - self.verts[0];

        let h = ray.dir.cross(&e2);
        let a = e1.dot(&h);

        if a.abs() < EPSILON {
            return false;
        }

        let f = 1.0 / a;
        let s = ray.pos - self.verts[0];
        let u = f * s.dot(&h);

        if (u < 0.0) || (u > 1.0) {
            return false;
        }

        let q = s.cross(&e1);
        let v = f * ray.dir.dot(&q);

        if (v < 0.0) || ((u + v) > 1.0) {
            return false;
        }

        let dist = f * e2.dot(&q);

        if dist < EPSILON {
            return false;
        }

        true
    }

    fn dist(&self, ray: &Ray) -> Option<f64> {
        let e1 = self.verts[1] - self.verts[0];
        let e2 = self.verts[2] - self.verts[0];

        let h = ray.dir.cross(&e2);
        let a = e1.dot(&h);

        if a.abs() < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.pos - self.verts[0];
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

        Some(dist)
    }

    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        let verts = self.verts;

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
