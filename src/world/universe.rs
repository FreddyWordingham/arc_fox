//! Universe structure.

use crate::{
    sci::chem::{Reaction, Species},
    world::{
        mat::{Interface, Material},
        parts::{interfaces, materials, reactions, species},
        UniverseBuilder,
    },
};
use self_ref::self_referencing;
use std::sync::Arc;

/// Universe structure implementation.
#[derive(Debug)]
pub struct Universe<'a> {
    /// Species present.
    species: Vec<Species>,
    /// Materials within.
    materials: Vec<Material>,
    /// Interfaces.
    interfaces: Vec<Interface<'a>>,
    /// Reactions happening.
    reactions: Vec<Reaction>,
}

impl<'a> Universe<'a> {
    /// Build a new instance.
    pub fn build(builder: UniverseBuilder) -> Self {
        Arc::try_unwrap(self_referencing!(Universe, {
            species = species::build(builder.species);
            materials = materials::build(builder.materials);
            interfaces = interfaces::build(builder.interfaces, &builder.meshes, &materials);
            reactions = reactions::build(builder.reactions, &species);
        }))
        .expect("Could not create universe instance.")
    }
}
