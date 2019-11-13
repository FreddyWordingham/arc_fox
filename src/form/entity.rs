//! Entity form structure.

use super::Transform;
use crate::json;
use contracts::pre;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Entity input form.
#[derive(Serialize, Deserialize)]
pub struct Entity {
    /// Identity string.
    pub id: String,
    /// Inside material id.
    pub in_mat: String,
    /// Outside material id
    pub out_mat: String,
    /// Surface mesh name.
    pub mesh: String,
    /// Optional transform.
    pub transform: Option<Transform>,
}

impl Entity {
    /// Construct a new instance.
    #[pre(!id.is_empty())]
    #[pre(!in_mat.is_empty())]
    #[pre(!out_mat.is_empty())]
    #[pre(!mesh.is_empty())]
    pub fn new(
        id: String,
        in_mat: String,
        out_mat: String,
        mesh: String,
        transform: Option<Transform>,
    ) -> Self {
        Self {
            id,
            in_mat,
            out_mat,
            mesh,
            transform,
        }
    }
}

json!(Entity);
