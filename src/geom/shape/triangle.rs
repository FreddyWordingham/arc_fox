//! Triangle structure.

use crate::list::alphabet::Greek::{Alpha, Beta, Gamma};
use contracts::pre;
use nalgebra::{Point3, Unit, Vector3};

/// Triangle structure implementation.
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
    #[pre(norms.iter().all(|n| (n.magnitude() - 1.0).abs() < 1.0e-6))]
    pub fn new(verts: [Point3<f64>; 3], norms: [Unit<Vector3<f64>>; 3]) -> Self {
        let plane_norm = Unit::new_normalize(
            (verts[Beta as usize] - verts[Alpha as usize])
                .cross(&(verts[Gamma as usize] - verts[Alpha as usize])),
        );

        if !norms.iter().all(|&n| n.dot(&plane_norm) > 0.0) {
            panic!("Normals are not in direction with the plane.");
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
