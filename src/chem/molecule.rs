//! Molecule structure.

use crate::json;
use contracts::pre;
use serde::{Deserialize, Serialize};

/// Molecule structure implementation.
#[derive(Debug, Serialize, Deserialize)]
pub struct Molecule {
    /// Radius of the molecule [m].
    rad: f64,
}

impl Molecule {
    /// Construct a new instance.
    #[pre(rad > 0.0)]
    pub fn new(rad: f64) -> Self {
        Self { rad }
    }
}

json!(Molecule);
