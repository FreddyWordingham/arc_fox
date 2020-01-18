//! Reaction builder structure.

use proc_mac::Json;
use serde::{Deserialize, Serialize};

/// Species builder structure.
#[derive(Debug, Deserialize, Serialize, Json)]
pub struct SpeciesBuilder {
    /// Optional diffusive radius [m].
    pub rad: Option<f64>,
}
