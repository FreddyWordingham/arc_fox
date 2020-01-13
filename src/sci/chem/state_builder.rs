//! State building structure.

use crate::rw_json;
use serde::{Deserialize, Serialize};

/// Reaction builder structure.
#[derive(Debug, Serialize, Deserialize)]
pub struct StateBuilder {
    /// Initial state of each species (name), concentration and source/sink term.
    pub init: Vec<(String, f64, f64)>,
}

impl StateBuilder {
    /// Construct a new instance.
    pub fn new(init: Vec<(String, f64, f64)>) -> Self {
        StateBuilder { init }
    }
}

rw_json!(StateBuilder);
