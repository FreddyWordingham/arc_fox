//! Geometric triangle structure.

use super::{Aabb, Collidable, Ray, Traceable};
use nalgebra::{Isometry3, Point3, Unit, Vector3};
use serde::{Deserialize, Serialize};

/// Parallel ray catch value.
const EPSILON: f64 = 1.0e-6;

/// Geometry describing a triangle in three-dimensional space.
/// Used to compose surfaces.
#[derive(Debug, Serialize, Deserialize)]
pub struct Triangle {
    /// Vertices.
    verts: [Point3<f64>; 3],
    /// Surface normal.
    norm: Unit<Vector3<f64>>,
}

impl Triangle {
    /// Construct a new instance.
    pub fn new(verts: [Point3<f64>; 3]) -> Self {
        Self {
            verts,
            norm: Unit::new_normalize((verts[1] - verts[0]).cross(&(verts[2] - verts[0]))),
        }
    }

    /// Reference the vertex positions.
    pub fn verts(&self) -> &[Point3<f64>; 3] {
        &self.verts
    }

    /// Reference the surface normal.
    pub fn norm(&self) -> &Unit<Vector3<f64>> {
        &self.norm
    }

    /// Calculate the area.
    pub fn area(&self) -> f64 {
        ((self.verts[1] - self.verts[0]).cross(&(self.verts[2] - self.verts[0]))).magnitude() / 2.0
    }

    /// Apply a transformation to the triangle.
    pub fn transform(&mut self, trans: &Isometry3<f64>) {
        self.norm = trans * self.norm;

        for v in self.verts.iter_mut() {
            *v = trans * *v;
        }
    }
}

impl Traceable for Triangle {
    fn intersect(&self, ray: &Ray) -> bool {
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

        Some((dist, self.norm))
    }
}

impl Collidable for Triangle {
    fn collides(&self, aabb: &Aabb) -> bool {
        let c = aabb.centre();
        let e = aabb.half_widths();

        let v0 = self.verts[0] - c;
        let v1 = self.verts[1] - c;
        let v2 = self.verts[2] - c;

        let f0 = v1 - v0;
        let f1 = v2 - v1;
        let f2 = v0 - v2;

        let u0 = Vector3::x_axis();
        let u1 = Vector3::y_axis();
        let u2 = Vector3::z_axis();

        let axis_test = |axis: &Vector3<f64>| {
            let p0 = v0.dot(axis);
            let p1 = v1.dot(axis);
            let p2 = v2.dot(axis);

            let r = (e.x * (u0.dot(axis)).abs())
                + (e.y * (u1.dot(axis)).abs())
                + (e.z * (u2.dot(axis)).abs());

            if (-(p0.max(p1).max(p2))).max(p0.min(p1).min(p2)) > r {
                return false;
            }

            true
        };

        if !axis_test(&u0) {
            return false;
        }
        if !axis_test(&u1) {
            return false;
        }
        if !axis_test(&u2) {
            return false;
        }

        let axis_u0_f0 = u0.cross(&f0);
        let axis_u0_f1 = u0.cross(&f1);
        let axis_u0_f2 = u0.cross(&f2);

        let axis_u1_f0 = u1.cross(&f0);
        let axis_u1_f1 = u1.cross(&f1);
        let axis_u1_f2 = u1.cross(&f2);

        let axis_u2_f0 = u2.cross(&f0);
        let axis_u2_f1 = u2.cross(&f1);
        let axis_u2_f2 = u2.cross(&f2);

        if !axis_test(&axis_u0_f0) {
            return false;
        }
        if !axis_test(&axis_u0_f1) {
            return false;
        }
        if !axis_test(&axis_u0_f2) {
            return false;
        }

        if !axis_test(&axis_u1_f0) {
            return false;
        }
        if !axis_test(&axis_u1_f1) {
            return false;
        }
        if !axis_test(&axis_u1_f2) {
            return false;
        }

        if !axis_test(&axis_u2_f0) {
            return false;
        }
        if !axis_test(&axis_u2_f1) {
            return false;
        }
        if !axis_test(&axis_u2_f2) {
            return false;
        }

        if !axis_test(&self.norm) {
            return false;
        }

        true
    }

    fn boundary(&self) -> Aabb {
        let mut mins = self.verts[0];
        let mut maxs = mins;

        for v in self.verts.iter().skip(1) {
            for i in 0..3 {
                if v[i] < mins[i] {
                    mins[i] = v[i];
                } else if v[i] > maxs[i] {
                    maxs[i] = v[i];
                }
            }
        }

        Aabb::new(mins, maxs)
    }
}
