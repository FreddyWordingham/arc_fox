//! Material structure.

use crate::world::{mat::MaterialBuilder, parts::Named};
use contracts::pre;

/// Material structure implementation.
/// Stores the local optical, diffusive and kinematic information.
#[derive(Debug)]
pub struct Material {
    /// Name of the material.
    name: String,
    /// Optional viscosity. [kg m s^-1]
    visc: Option<f64>,
    /// Reaction rate multiplier.
    reaction_multiplier: f64,
}

impl Material {
    /// Construct a new instance.
    #[pre(!name.is_empty())]
    #[pre(visc.is_none() || visc.unwrap() > 0.0)]
    #[pre(reaction_multiplier > 0.0)]
    pub fn new(name: String, visc: Option<f64>, reaction_multiplier: f64) -> Self {
        Self {
            name,
            visc,
            reaction_multiplier,
        }
    }

    /// Build a new instance.
    #[pre(!name.is_empty())]
    pub fn build(name: String, builder: MaterialBuilder) -> Self {
        Self::new(name, builder.visc, builder.reaction_multiplier)
    }

    /// Get the viscosity.
    pub fn visc(&self) -> Option<f64> {
        self.visc
    }

    /// Get the reaction rate multiplier.
    pub fn reaction_multiplier(&self) -> f64 {
        self.reaction_multiplier
    }
}

impl Named for Material {
    /// Reference the name.
    fn name(&self) -> &str {
        &self.name
    }
}
