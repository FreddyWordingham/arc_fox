//! Chemical species information structure.

use crate::file::{as_json, from_json, Loadable, Saveable};
use contracts::pre;
use serde::{Deserialize, Serialize};
use std::path::Path;

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

impl Saveable for Species {
    fn save(&self, path: &Path) {
        as_json(self, path);
    }
}

impl Loadable for Species {
    fn load(path: &Path) -> Self {
        from_json(path)
    }
}
