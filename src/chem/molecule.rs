//! Molecule structure.

use crate::{json, world::Identity};
use contracts::pre;
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
    #[pre(!id.is_empty())]
    #[pre(rad > 0.0)]
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
