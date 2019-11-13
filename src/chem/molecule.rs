//! Molecule structure.

use crate::{json, world::Identity};
use serde::{Deserialize, Serialize};

/// Chemical molecule structure.
#[derive(Serialize, Deserialize)]
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

json!(Molecule);
