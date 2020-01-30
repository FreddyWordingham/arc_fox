//! Mesh implementation.

use crate::{
    file::{Load, Transform},
    geom::{Mesh as GeomMesh, Transform as GeomTransform},
};
use attr::json;
use std::path::Path;

/// Mesh construction form.
#[json]
pub struct Surface {
    /// Base mesh name.
    mesh: String,
    /// Optional transform to apply.
    trans: Option<Transform>,
}

impl Surface {
    /// Build a mesh.
    #[inline]
    #[must_use]
    pub fn build(&self, in_dir: &Path) -> GeomMesh {
        let mut mesh = GeomMesh::load(&in_dir.join(format!("{}.obj", self.mesh)));

        if let Some(trans) = &self.trans {
            mesh.transform(&trans.build());
        }

        mesh
    }
}
