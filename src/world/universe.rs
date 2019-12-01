//! Universe structure.

#![allow(unused_variables)]
#![allow(clippy::mem_forget)]

use crate::{
    sci::chem::{Reaction, Species},
    world::{
        dom::Grid,
        mat::{Interface, Material},
        parts::{interfaces, materials, reactions, species},
        UniverseBuilder,
    },
};
use contracts::pre;
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
    /// Grid.
    grid: Grid,
}

impl<'a> Universe<'a> {
    /// Build a new instance.
    #[pre(num_threads > 0)]
    pub fn build(num_threads: usize, builder: UniverseBuilder) -> Self {
        Arc::try_unwrap(self_referencing!(Universe, {
            species = species::build(builder.species);
            materials = materials::build(builder.materials, species);
            interfaces = interfaces::build(builder.interfaces, &builder.meshes, materials);
            reactions = reactions::build(builder.reactions, species);
            grid = Grid::new(num_threads, builder.res, builder.dom);
        }))
        .expect("Could not create universe instance.")
    }

    /// Reference the species.
    pub fn species(&self) -> &Vec<Species> {
        &self.species
    }

    /// Reference the materials.
    pub fn materials(&self) -> &Vec<Material> {
        &self.materials
    }

    /// Reference the interfaces.
    pub fn interfaces(&self) -> &Vec<Interface<'a>> {
        &self.interfaces
    }

    /// Reference the reactions.
    pub fn reactions(&self) -> &Vec<Reaction> {
        &self.reactions
    }
}
