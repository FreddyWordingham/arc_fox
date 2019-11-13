//! Molecule structure.

use crate::world::Identity;

/// Chemical molecule structure.
pub struct Molecule {
    /// Identification string.
    id: String,
    /// Radius [m].
    rad: f64,
}

impl Molecule {
    /// Construct a new instance.
    pub fn new(id: String, rad: f64) -> Self {
        Self { id, rad }
    }
}

impl Identity for Molecule {
    fn id(&self) -> &str {
        &self.id
    }
}
