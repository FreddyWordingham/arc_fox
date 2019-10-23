//! Chemical species information structure.

use contracts::pre;
use serde::{Deserialize, Serialize};

/// Chemical species information.
#[derive(Debug, Deserialize, Serialize)]
pub struct Species {
    /// Molecule radius.
    radius: f64,
}

impl Species {
    /// Construct a new instance.
    #[pre(radius > 0.0)]
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
}
