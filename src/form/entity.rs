//! Entity form structure.

use super::Transform;
use crate::json;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Entity input form.
#[derive(Serialize, Deserialize)]
pub struct Entity {
    /// Optional transform.
    transform: Option<Transform>,
}

impl Entity {
    /// Construct a new instance.
    pub fn new(transform: Option<Transform>) -> Self {
        Self { transform }
    }
}

json!(Entity);
