//! World builder input form.

use serde::{Deserialize, Serialize};

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
    /// Construct a new instance.
    pub fn new(mins: [f64; 3], maxs: [f64; 3], num_cells: [usize; 3]) -> Self {
        Self {
            mins,
            maxs,
            num_cells,
        }
    }
}
