//! Interface structure.

use crate::{geom::shape::Mesh, mat::Material};

/// Interface structure implementation.
/// Forms the boundary between two regions of material.
#[derive(Debug)]
pub struct Interface<'a> {
    /// Surface mesh.
    mesh: Mesh,
    /// Inside material.
    in_mat: &'a Material,
    /// Outside material.
    out_mat: &'a Material,
}

impl<'a> Interface<'a> {
    /// Construct a new instance.
    pub fn new(mesh: Mesh, in_mat: &'a Material, out_mat: &'a Material) -> Self {
        Self {
            mesh,
            in_mat,
            out_mat,
        }
    }
}
