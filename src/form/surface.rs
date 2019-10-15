//! Surface model input.

use serde::{Deserialize, Serialize};

/// Surface setup information.
#[derive(Debug, Deserialize, Serialize)]
pub struct Surface {
    /// Mesh surface.
    mesh: String,
    /// Inside material.
    inside: String,
    /// Outside material.
    outside: String,
}

impl Surface {
    /// Construct a new instance.
    pub fn new(mesh: String, inside: String, outside: String) -> Self {
        Self {
            mesh,
            inside,
            outside,
        }
    }
}
