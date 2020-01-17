//! Triangle geometry structure.

use crate::util::list::alphabet::Greek::{Alpha, Beta, Gamma};
use nalgebra::{Point3, Unit, Vector3};

/// Triangle structure implementation.
/// Forms meshes.
#[derive(Clone)]
pub struct Triangle {
    /// Vertex points.
    pub verts: [Point3<f64>; 3],
    /// Normal vectors.
    pub norms: [Unit<Vector3<f64>>; 3],
    /// Surface plane normal.
    pub plane_norm: Unit<Vector3<f64>>,
}

impl Triangle {
    /// Construct a new instance.
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
}
