//! Mesh-Builder structure.

use crate::{json, sci::math::geom::TransformBuilder};
use serde::{Deserialize, Serialize};

/// Mesh-Builder structure implementation.
/// Used to build meshes.
#[derive(Debug, Deserialize, Serialize)]
pub struct MeshBuilder {
    /// Mesh name.
    pub name: String,
    /// Optional transform.
    pub trans: Option<TransformBuilder>,
}

json!(MeshBuilder);
