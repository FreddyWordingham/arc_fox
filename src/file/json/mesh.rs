//! Mesh implementation.

use crate::{
    file::{Load, Transform},
    geom::{Mesh as GeomMesh, Transform as GeomTransform},
};
use attr::json;
use std::path::Path;

/// Mesh construction form.
#[json]
pub struct Mesh {
    /// Base mesh name.
    name: String,
    /// Optional transform to apply.
    trans: Option<Transform>,
}

impl Mesh {
    /// Build a mesh.
    #[inline]
    #[must_use]
    pub fn build(&self, in_dir: &Path) -> GeomMesh {
        let mut mesh = GeomMesh::load(&in_dir.join(format!("{}.obj", self.name)));

        if let Some(trans) = &self.trans {
            mesh.transform(&trans.build());
        }

        mesh
    }
}
