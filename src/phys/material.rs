//! Physical material structure.

use crate::math::Formula;
use serde::{Deserialize, Serialize};

/// Physical material structure.
#[derive(Debug, Serialize, Deserialize)]
pub struct Material {
    /// Refractive index.
    ref_index: Formula,
}

impl Material {
    /// Construct a new instance.
    pub fn new(ref_index: Formula) -> Self {
        Self { ref_index }
    }
}
