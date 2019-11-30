//! Smooth-triangle structure.

use super::super::{geom::Collide, shape::Aabb};
use crate::util::list::alphabet::Greek::{Alpha, Beta, Gamma};
use contracts::pre;
use nalgebra::{Point3, Unit, Vector3};

/// Triangle structure implementation.
/// Forms meshes.
#[derive(Debug)]
pub struct Triangle {
    /// Vertex points.
    verts: [Point3<f64>; 3],
    /// Normal vectors.
    norms: [Unit<Vector3<f64>>; 3],
    /// Surface plane normal.
    plane_norm: Unit<Vector3<f64>>,
}

impl Triangle {
    /// Construct a new instance.
    #[pre(norms.iter().all(|n| (n.magnitude_squared() - 1.0).abs() < 1.0e-6))]
    pub fn new(verts: [Point3<f64>; 3], norms: [Unit<Vector3<f64>>; 3]) -> Self {
        let plane_norm = Self::init_plane_norm(&verts);

        if !norms.iter().all(|&n| n.dot(&plane_norm) > 0.0) {
            panic!("Normals are not in direction with the plane.");
        }

        Self {
            verts,
            norms,
            plane_norm,
        }
    }

    /// Initialise the plane normal.
    fn init_plane_norm(verts: &[Point3<f64>; 3]) -> Unit<Vector3<f64>> {
        Unit::new_normalize(
            (verts[Beta as usize] - verts[Alpha as usize])
                .cross(&(verts[Gamma as usize] - verts[Alpha as usize])),
        )
    }

    /// Reference the vertices.
    pub fn verts(&self) -> &[Point3<f64>; 3] {
        &self.verts
    }

    /// Reference the normal vectors.
    pub fn norms(&self) -> &[Unit<Vector3<f64>>; 3] {
        &self.norms
    }

    /// Point on the surface which is also within the given aabb.
    pub fn union_point(&self, aabb: &Aabb) -> Option<Point3<f64>> {
        if !self.overlap(aabb) {
            return None;
        }

        let su = self.verts[Beta as usize] - self.verts[Alpha as usize];
        let sv = self.verts[Gamma as usize] - self.verts[Alpha as usize];

        for power in 1..5 {
            let n = 2i32.pow(power);
            let df = 1.0 / n as f64;
            for ui in 1..n {
                let mut u = df * ui as f64;
                for vi in 1..n {
                    let mut v = df * vi as f64;

                    if (u + v) > 1.0 {
                        u = 1.0 - u;
                        v = 1.0 - v;
                    }
                    if (u + v) > 1.0 {
                        panic!("Didn't work!"); // TODO: Remove.
                    }
                    let p: Point3<f64> = self.verts[Alpha as usize] + (su * u) + (sv * v);

                    if aabb.contains(&p) {
                        return Some(p);
                    }
                }
            }
        }

        None
    }
}

impl Collide for Triangle {
    fn bounding_box(&self) -> Aabb {
        let mut mins = self.verts[Alpha as usize];
        let mut maxs = mins;

        for v in self.verts.iter().skip(1) {
            for (v, (min, max)) in v.iter().zip(mins.iter_mut().zip(maxs.iter_mut())) {
                if *min > *v {
                    *min = *v;
                } else if { *max < *v } {
                    *max = *v;
                }
            }
        }

        Aabb::new(mins, maxs)
    }

    fn overlap(&self, aabb: &Aabb) -> bool {
        let c = aabb.centre();
        let e = aabb.half_widths();

        let v0 = self.verts[Alpha as usize] - c;
        let v1 = self.verts[Beta as usize] - c;
        let v2 = self.verts[Gamma as usize] - c;

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
