//! Interface-Builder structure.

use crate::{json, sci::math::shape::MeshBuilder};
use serde::{Deserialize, Serialize};

/// Interface-Builder structure implementation.
/// Used to build interfaces.
#[derive(Debug, Deserialize, Serialize)]
pub struct InterfaceBuilder {
    /// Mesh.
    pub mesh: MeshBuilder,
    /// Inside material name.
    pub in_mat: String,
    /// Outside material name.
    pub out_mat: String,
}

json!(InterfaceBuilder);
