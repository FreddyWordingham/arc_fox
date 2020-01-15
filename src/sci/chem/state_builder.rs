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
    #[inline]
    pub const fn new(init: Vec<(String, f64, f64)>) -> Self {
        Self { init }
    }
}

rw_json!(StateBuilder);
