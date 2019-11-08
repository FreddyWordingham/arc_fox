//! Triangle structure.

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
