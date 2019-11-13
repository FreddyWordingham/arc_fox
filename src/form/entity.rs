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
    id: String,
    /// Optional transform.
    transform: Option<Transform>,
}

impl Entity {
    /// Construct a new instance.
    #[pre(!id.is_empty())]
    pub fn new(id: String, transform: Option<Transform>) -> Self {
        Self { id, transform }
    }
}

json!(Entity);
