//! Parameters structure.

use crate::json;
use contracts::pre;
use serde::{Deserialize, Serialize};

/// Parameters structure implementation.
/// Load-time world building information.
#[derive(Debug, Deserialize, Serialize)]
pub struct Parameters {
    /// Number threads to use.
    num_threads: usize,
    /// List of reactions to simulate.
    reactions: Vec<String>,
}

impl Parameters {
    /// Construct a new instance.
    #[pre(num_threads > 0)]
    pub fn new(num_threads: usize, reactions: Vec<String>) -> Self {
        Self {
            num_threads,
            reactions,
        }
    }
}

json!(Parameters);
