//! World builder input form.

use crate::file::{as_json, from_json, Loadable, Saveable};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Input form structure containing all information required to run the world_builder binary.
#[derive(Debug, Deserialize, Serialize)]
pub struct WorldBuilder {
    /// Minimum bound.
    mins: [f64; 3],
    /// Maximum bound.
    maxs: [f64; 3],
    /// Number of cells.
    num_cells: [usize; 3],
}

impl WorldBuilder {
    /// Create an example world.
    pub fn example() -> Self {
        Self {
            mins: [-1.0, -1.0, -1.0],
            maxs: [1.0, 1.0, 1.0],
            num_cells: [8, 8, 8],
        }
    }
}

impl Saveable for WorldBuilder {
    fn save(&self, path: &Path) {
        as_json(self, path);
    }
}

impl Loadable for WorldBuilder {
    fn load(path: &Path) -> Self {
        from_json(path)
    }
}
