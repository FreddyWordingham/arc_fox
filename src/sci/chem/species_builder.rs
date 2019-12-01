//! Species-Builder structure.

use crate::json;
use serde::{Deserialize, Serialize};

/// Species-Builder structure implementation.
/// Used to build species.
#[derive(Debug, Deserialize, Serialize)]
pub struct SpeciesBuilder {
    /// Optional radius of the molecule [m].
    pub rad: Option<f64>,
}

json!(SpeciesBuilder);
