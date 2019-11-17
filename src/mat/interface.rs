//! Interface structure.

use crate::{
    geom::shape::{Mesh, ProtoMesh},
    json,
    mat::Material,
    world::MatMap,
};
use contracts::pre;
use serde::{Deserialize, Serialize};

/// Interface structure implementation.
/// Forms the boundary between two regions of material.
#[derive(Debug)]
pub struct Interface<'a> {
    /// Inside material.
    in_mat: &'a Material,
    /// Outside material.
    out_mat: &'a Material,
    /// Surface mesh.
    mesh: Mesh,
}

impl<'a> Interface<'a> {
    /// Construct a new instance.
    pub fn new(in_mat: &'a Material, out_mat: &'a Material, mesh: Mesh) -> Self {
        Self {
            in_mat,
            out_mat,
            mesh,
        }
    }

    /// Build an instance from a proto-interface.
    #[pre(mesh_dir.is_dir())]
    #[pre(!mat_map.is_empty())]
    pub fn build(mesh_dir: &Path, proto_inter: &ProtoInterface, mat_map: &'a MatMap) -> Self {
        Self::new(
            &mat_map[proto_inter.in_mat()],
            &mat_map[proto_inter.out_mat()],
            Mesh::build(mesh_dir, proto_inter.mesh()),
        )
    }
}

/// Proto-Interface structure implementation.
/// Stores information required to build an interface.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProtoInterface {
    /// Proto-mesh.
    mesh: ProtoMesh,
    /// Inside material key.
    in_mat: String,
    /// Outside material key.
    out_mat: String,
}

impl ProtoInterface {
    /// Construct a new instance.
    #[pre(!in_mat.is_empty())]
    #[pre(!out_mat.is_empty())]
    pub fn new(mesh: ProtoMesh, in_mat: String, out_mat: String) -> Self {
        Self {
            mesh,
            in_mat,
            out_mat,
        }
    }

    /// Reference the mesh.
    pub fn mesh(&self) -> &ProtoMesh {
        &self.mesh
    }

    /// Reference the inside material.
    pub fn in_mat(&self) -> &str {
        &self.in_mat
    }

    /// Reference the outside material.
    pub fn out_mat(&self) -> &str {
        &self.out_mat
    }
}

json!(ProtoInterface);
