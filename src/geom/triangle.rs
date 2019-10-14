//! Geometric triangle structure with interpolated phong normals.

use nalgebra::{Point3, Unit, Vector3};

/// Triangle in three-dimensions.
/// Commonly used to compose triangle-meshes which in turn form surfaces.
#[derive(Debug)]
pub struct Triangle {
    /// Vertex positions.
    verts: [Point3<f64>; 3],
    /// Vertex normal vectors.
    norms: [Unit<Vector3<f64>>; 3],
}

impl Triangle {
    /// Construct a new instance.
    pub fn new(verts: [Point3<f64>; 3], norms: [Unit<Vector3<f64>>; 3]) -> Self {
        Self { verts, norms }
    }

    /// Reference the vertex positions.
    pub fn verts(&self) -> &[Point3<f64>; 3] {
        &self.verts
    }

    /// Reference the vertex normals.
    pub fn norms(&self) -> &[Unit<Vector3<f64>>; 3] {
        &self.norms
    }
}
