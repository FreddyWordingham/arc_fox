//! Mesh implementation.

use crate::file::Transform;
use attr::json;

/// Mesh construction form.
#[json]
pub struct Mesh {
    /// Base mesh name.
    name: String,
    /// Optional transform to apply.
    trans: Option<Transform>,
}

impl Mesh {
    // pub fn build(&self) -> crate::geom::Mesh {}
}
