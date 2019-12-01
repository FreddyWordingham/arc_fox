//! Material structure.

use crate::world::{mat::MaterialBuilder, parts::Named};
use contracts::pre;

/// Material structure implementation.
/// Stores the local optical, diffusive and kinematic information.
#[derive(Debug)]
pub struct Material {
    /// Name of the material.
    name: String,
}

impl Material {
    /// Construct a new instance.
    #[pre(!name.is_empty())]
    pub fn new(name: String) -> Self {
        Self { name }
    }

    /// Build a new instance.
    #[pre(!name.is_empty())]
    pub fn build(name: String, _builder: MaterialBuilder) -> Self {
        Self::new(name)
    }
}

impl Named for Material {
    /// Reference the name.
    fn name(&self) -> &str {
        &self.name
    }
}
