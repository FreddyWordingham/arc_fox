//! Mesh structure.

use super::super::ProtoTransform;
use crate::json;
use contracts::pre;
use serde::{Deserialize, Serialize};

/// Mesh structure implementation.
#[derive(Debug)]
pub struct Mesh {
    // Fields.
}

impl Mesh {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}

/// Proto-Transform structure implementation.
/// Stores information required to build a mesh.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProtoMesh {
    /// Mesh name.
    name: String,
    /// Optional transform.
    trans: Option<ProtoTransform>,
}

impl ProtoMesh {
    /// Construct a new instance.
    #[pre(!name.is_empty())]
    pub fn new(name: String, trans: Option<ProtoTransform>) -> Self {
        Self { name, trans }
    }

    /// Build a mesh.
    pub fn build(&self) -> Mesh {
        Mesh::new()
    }
}

json!(ProtoMesh);
