//! State building structure.

use crate::rw_json;
use serde::{Deserialize, Serialize};

/// Reaction builder structure.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StateBuilder {
    /// Initial state of species concentration.
    pub concs: Option<Vec<(String, f64)>>,
    /// Initial state of species source/sink terms.
    pub sources: Option<Vec<(String, f64)>>,
}

impl StateBuilder {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        concs: Option<Vec<(String, f64)>>,
        sources: Option<Vec<(String, f64)>>,
    ) -> Self {
        Self { concs, sources }
    }
}

rw_json!(StateBuilder);
