//! Triangle structure.

use super::{Aabb, Collision, Touchable, EPSILON};
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

    /// Reference the vertices.
    pub fn verts(&self) -> &[Point3<f64>; 3] {
        &self.verts
    }

    /// Reference the normal vectors.
    pub fn norms(&self) -> &[Unit<Vector3<f64>>; 3] {
        &self.norms
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

impl Collision for Triangle {
    fn bounding_box(&self) -> Aabb {
        let mut mins = self.verts[0];
        let mut maxs = mins;

        for v in self.verts.iter() {
            for i in 0..3 {
                if mins[i] > v[i] {
                    mins[i] = v[i];
                } else if maxs[i] < v[i] {
                    maxs[i] = v[i];
                }
            }
        }

        Aabb::new(mins, maxs)
    }

    fn overlap(&self, aabb: &Aabb) -> bool {
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

        if !axis_test(&self.plane_norm) {
            return false;
        }

        true
    }
}

impl Touchable for Triangle {
    fn closest_point(&self, p: &Point3<f64>) -> Point3<f64> {
        let a = self.verts[0];
        let b = self.verts[1];
        let c = self.verts[2];

        let ab = b - a;
        let ac = c - a;
        let ap = p - a;

        let d1 = ab.dot(&ap);
        let d2 = ac.dot(&ap);

        if (d1 <= 0.0) && (d2 < 0.0) {
            return a;
        }

        let bp = p - b;
        let d3 = ab.dot(&bp);
        let d4 = ac.dot(&bp);

        if (d3 >= 0.0) && (d4 <= d3) {
            return b;
        }

        let vc = (d1 * d4) - (d3 * d2);
        if (vc <= 0.0) && (d1 >= 0.0) && (d3 <= 0.0) {
            let v = d1 / (d1 - d3);
            return a + (v * ab);
        }

        let cp = p - c;
        let d5 = ab.dot(&cp);
        let d6 = ac.dot(&cp);

        if (d6 >= 0.0) && (d5 <= d6) {
            return c;
        }

        let vb = (d5 * d2) - (d1 * d6);
        if (vb <= 0.0) && (d2 >= 0.0) && (d6 <= 0.0) {
            let w = d2 / (d2 - d6);
            return a + (w * ac);
        }

        let va = (d3 * d6) + (d5 * d4);
        if (va <= 0.0) && ((d4 - d3) >= 0.0) && ((d5 - d6) >= 0.0) {
            let w = (d4 - d3) / ((d4 - d3) + (d5 - d6));
            return b + (w * (c - b));
        }

        let denom = 1.0 / (va + vb + vc);
        let v = vb * denom;
        let w = vc * denom;

        return a + (ab * v) + (ac * w);
    }
}
