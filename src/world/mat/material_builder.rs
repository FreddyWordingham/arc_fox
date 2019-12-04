//! Material-Builder structure.

use crate::{
    json,
    world::{dom::StateBuilder, mat::Optics},
};
use serde::{Deserialize, Serialize};

/// Material-Builder structure implementation.
/// Used to build materials.
#[derive(Debug, Deserialize, Serialize)]
pub struct MaterialBuilder {
    /// Optional viscosity. [kg m s^-1]
    pub visc: Option<f64>,
    /// Reaction rate multiplier.
    pub reaction_multiplier: f64,
    /// Optical properties.
    pub optics: Optics,
    /// Optional initial state.
    pub state: Option<StateBuilder>,
}

json!(MaterialBuilder);
