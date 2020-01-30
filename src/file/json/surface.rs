//! Mesh implementation.

use crate::{
    file::{Load, Transform},
    geom::{Mesh as GeomMesh, Transform as GeomTransform},
};
use attr::json;
use log::info;
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
    pub fn build(&self, dir: &Path, ext: &str) -> GeomMesh {
        let path = &dir.join(format!("{}.{}", self.mesh, ext));
        info!("Loading: {}", path.display());
        let mut mesh = GeomMesh::load(path);

        if let Some(trans) = &self.trans {
            mesh.transform(&trans.build());
        }

        mesh
    }
}
