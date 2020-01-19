//! Triangle geometry structure.

use crate::{
    access,
    sci::math::rt::ray::Ray,
    util::list::alphabet::Greek::{Alpha, Beta, Gamma},
};
use nalgebra::{Point3, Unit, Vector3};

/// Triangle structure implementation.
/// Forms meshes.
pub struct Triangle {
    /// Vertex points.
    verts: [Point3<f64>; 3],
    /// Surface plane normal.
    plane_norm: Unit<Vector3<f64>>,
}

impl Triangle {
    access!(verts, [Point3<f64>; 3]);
    access!(plane_norm, Unit<Vector3<f64>>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(verts: [Point3<f64>; 3]) -> Self {
        let plane_norm = Unit::new_normalize(
            (verts.get(Beta as usize).expect("Invalid vertex index.")
                - verts.get(Alpha as usize).expect("Invalid vertex index."))
            .cross(
                &(verts.get(Gamma as usize).expect("Invalid vertex index.")
                    - verts.get(Alpha as usize).expect("Invalid vertex index.")),
            ),
        );

        Self { verts, plane_norm }
    }

    /// Calculate the perimeter length.
    #[inline]
    #[must_use]
    pub fn perimeter(&self) -> f64 {
        let ab = nalgebra::distance(&self.verts[0], &self.verts[1]);
        let bc = nalgebra::distance(&self.verts[1], &self.verts[2]);
        let ca = nalgebra::distance(&self.verts[2], &self.verts[0]);

        ab + bc + ca
    }

    /// Calculate the surface area.
    #[inline]
    #[must_use]
    pub fn area(&self) -> f64 {
        let ab = nalgebra::distance(&self.verts[0], &self.verts[1]);
        let bc = nalgebra::distance(&self.verts[1], &self.verts[2]);
        let ca = nalgebra::distance(&self.verts[2], &self.verts[0]);

        let s = (ab + bc + ca) * 0.5;

        (s * (s - ab) * (s - bc) * (s - ca)).sqrt()
    }

    /// Centre point.
    #[inline]
    #[must_use]
    pub fn centre(&self) -> Point3<f64> {
        Point3::from(
            ((self
                .verts
                .get(Alpha as usize)
                .expect("Invalid vertex index")
                .to_homogeneous()
                + self
                    .verts
                    .get(Beta as usize)
                    .expect("Invalid vertex index")
                    .to_homogeneous()
                + self
                    .verts
                    .get(Gamma as usize)
                    .expect("Invalid vertex index")
                    .to_homogeneous())
                / 3.0)
                .xyz(),
        )
    }

    /// Determine the intersection distances along a ray's direction.
    /// Also return the barycentric intersection coordinates.
    #[inline]
    fn intersection_coors(&self, ray: &Ray) -> Option<(f64, [f64; 3])> {
        let verts = self.verts;

        let e1 = verts.get(Beta as usize).expect("Invalid vertex index.")
            - verts.get(Alpha as usize).expect("Invalid vertex index.");
        let e2 = verts.get(Gamma as usize).expect("Invalid vertex index.")
            - verts.get(Alpha as usize).expect("Invalid vertex index.");

        let d_cross_e2 = ray.dir().cross(&e2);
        let e1_dot_d_cross_e2 = e1.dot(&d_cross_e2);

        if e1_dot_d_cross_e2.abs() <= 0.0 {
            return None;
        }

        let inv_e1_dot_d_cross_e2 = 1.0 / e1_dot_d_cross_e2;
        let rel_pos = ray.pos() - verts.get(Alpha as usize).expect("Invalid vertex index.");
        let u = inv_e1_dot_d_cross_e2 * rel_pos.dot(&d_cross_e2);

        if (u < 0.0) || (u > 1.0) {
            return None;
        }

        let q = rel_pos.cross(&e1);
        let v = inv_e1_dot_d_cross_e2 * ray.dir().dot(&q);

        if (v < 0.0) || ((u + v) > 1.0) {
            return None;
        }

        let dist = inv_e1_dot_d_cross_e2 * e2.dot(&q);

        if dist <= 0.0 {
            return None;
        }

        let w = 1.0 - (u + v);

        Some((dist, [u, v, w]))
    }
}
