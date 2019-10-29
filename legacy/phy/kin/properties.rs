//! Kinetic statistical properties structure.

use crate::file::{as_json, from_json, Loadable, Saveable};
use contracts::pre;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Kinetic statistical physical properties.
/// Contains parameters governing how number density changes over space with time.
#[derive(Debug, Deserialize, Serialize)]
pub struct Properties {
    /// Diffusion coefficient [m^2/t].
    diff_coeff: f64,
}

impl Properties {
    /// Construct a new instance.
    #[pre(0.0 < diff_coeff)]
    pub fn new(diff_coeff: f64) -> Self {
        Self { diff_coeff }
    }

    /// Get the diffusion coefficient.
    pub fn diff_coeff(&self) -> f64 {
        self.diff_coeff
    }
}

impl Saveable for Properties {
    fn save(&self, path: &Path) {
        as_json(self, path);
    }
}

impl Loadable for Properties {
    fn load(path: &Path) -> Self {
        from_json(path)
    }
}
