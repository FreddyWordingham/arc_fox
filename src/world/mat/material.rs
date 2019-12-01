//! Material structure.

use crate::{
    sci::chem::Species,
    world::{dom::State, mat::MaterialBuilder, parts::Named},
};
use contracts::pre;
use ndarray::Array1;

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
    /// Initial state.
    init_state: State,
}

impl Material {
    /// Construct a new instance.
    #[pre(!name.is_empty())]
    #[pre(visc.is_none() || visc.unwrap() > 0.0)]
    #[pre(reaction_multiplier > 0.0)]
    pub fn new(
        name: String,
        visc: Option<f64>,
        reaction_multiplier: f64,
        init_state: State,
    ) -> Self {
        Self {
            name,
            visc,
            reaction_multiplier,
            init_state,
        }
    }

    /// Build a new instance.
    #[pre(!name.is_empty())]
    pub fn build(name: String, builder: MaterialBuilder, species: &[Species]) -> Self {
        let state = if let Some(state) = builder.state {
            State::build(state, species)
        } else {
            State::new(Array1::zeros(species.len()), Array1::zeros(species.len()))
        };

        Self::new(name, builder.visc, builder.reaction_multiplier, state)
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
