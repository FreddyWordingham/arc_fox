//! World entity structure.

use super::{Identity, Material};
use crate::geom::Mesh;
use contracts::{post, pre};

/// World entity structure.
/// Binds a material to a shape.
#[derive(Debug)]
pub struct Entity<'a> {
    /// Id string.
    id: String,
    /// Surface mesh.
    mesh: Mesh,
    /// Inside material.
    in_mat: &'a Material,
    /// Outside material.
    out_mat: &'a Material,
}

impl<'a> Entity<'a> {
    /// Construct a new instance.
    #[pre(!id.is_empty())]
    pub fn new(id: String, mesh: Mesh, in_mat: &'a Material, out_mat: &'a Material) -> Self {
        Self {
            id,
            mesh,
            in_mat,
            out_mat,
        }
    }

    /// Reference the surface mesh.
    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    /// Reference the inside material.
    pub fn in_mat(&self) -> &Material {
        &self.in_mat
    }

    /// Reference the outside material.
    pub fn out_mat(&self) -> &Material {
        &self.out_mat
    }
}

impl<'a> Identity for Entity<'a> {
    #[post(!ret.is_empty())]
    fn id(&self) -> &str {
        &self.id
    }
}
