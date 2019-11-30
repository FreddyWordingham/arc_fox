//! Interface structure.

use crate::{sci::math::shape::Mesh, world::mat::Material};

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
    pub const fn new(mesh: Mesh, in_mat: &'a Material, out_mat: &'a Material) -> Self {
        Self {
            mesh,
            in_mat,
            out_mat,
        }
    }

    /// Reference the surface mesh.
    pub const fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    /// Reference the inside material.
    pub const fn in_mat(&self) -> &Material {
        self.in_mat
    }

    /// Reference the outside material.
    pub const fn out_mat(&self) -> &Material {
        self.out_mat
    }
}
