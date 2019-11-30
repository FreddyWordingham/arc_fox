//! Material-Builder structure.

use crate::json;
use serde::{Deserialize, Serialize};

/// Material-Builder structure implementation.
/// Used to build materials.
#[derive(Debug, Deserialize, Serialize)]
pub struct MaterialBuilder {
    /// Optional viscosity. [kg m s^-1]
    visc: Option<f64>,
    /// Reaction rate multiplier.
    reaction_multiplier: f64,
}

json!(MaterialBuilder);
