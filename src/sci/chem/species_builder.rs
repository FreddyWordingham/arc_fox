//! Reaction builder structure.

use crate::rw_json;
use serde::{Deserialize, Serialize};

/// Species builder structure.
#[derive(Debug, Deserialize, Serialize)]
pub struct SpeciesBuilder {
    /// Optional diffusive radius [m].
    pub rad: Option<f64>,
}

rw_json!(SpeciesBuilder);
