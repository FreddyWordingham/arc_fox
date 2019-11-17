//! Molecule structure.

use crate::json;
use contracts::pre;
use serde::{Deserialize, Serialize};

/// Molecule structure implementation.
#[derive(Debug, Serialize, Deserialize)]
pub struct Molecule {
    /// Optional of the molecule [m].
    rad: Option<f64>,
}

impl Molecule {
    /// Construct a new instance.
    #[pre(rad.is_none() || rad.unwrap() > 0.0)]
    pub fn new(rad: Option<f64>) -> Self {
        Self { rad }
    }
}

json!(Molecule);
