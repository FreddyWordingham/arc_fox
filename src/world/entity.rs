//! World entity structure.

use crate::geom::Shape;
use contracts::pre;

/// World entity structure.
/// Binds a material to a geometry.
pub struct Entity {
    /// Surface geometry.
    surfs: Vec<Box<dyn Shape>>,
}

impl Entity {
    /// Construct a new instance.
    #[pre(!surfs.is_empty())]
    pub fn new(surfs: Vec<Box<dyn Shape>>) -> Self {
        Self { surfs }
    }

    /// Access the surface list.
    pub fn surfs(&self) -> &Vec<Box<dyn Shape>> {
        &self.surfs
    }
}
