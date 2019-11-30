//! Mesh-Builder structure.

use crate::{json, sci::math::geom::TransformBuilder};
use serde::{Deserialize, Serialize};

/// Mesh-Builder structure implementation.
/// Used to build meshes.
#[derive(Debug, Deserialize, Serialize)]
pub struct MeshBuilder {
    /// Mesh name.
    name: String,
    /// Optional transform.
    trans: Option<TransformBuilder>,
}

json!(MeshBuilder);
