//! Universe structure.

#![allow(unused_variables)]
#![allow(clippy::mem_forget)]

use crate::{
    sci::chem::{Reaction, Species},
    world::{
        dom::Grid,
        mat::{Interface, Material},
        parts::{interfaces, materials, reactions, species, Named},
        UniverseBuilder,
    },
};
use contracts::pre;
use ndarray::Array3;
use self_ref::self_referencing;
use std::{cmp::Ordering, sync::Arc};

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
    grid: Grid<'a>,
    /// Bump distance.
    bump_dist: f64,
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
            grid = Grid::new(num_threads, builder.res, builder.dom, interfaces);
            bump_dist = grid.cells()[(0, 0, 0)]
                .boundary()
                .widths()
                .iter()
                .min_by(|a, b| {
                    if a < b {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                })
                .unwrap()
                / 1000.0;
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

    /// Reference the grid.
    pub fn grid(&self) -> &Grid<'a> {
        &self.grid
    }

    /// Reference the grid mutably.
    pub fn grid_mut(&mut self) -> &mut Grid<'a> {
        &mut self.grid
    }

    /// Get the bump distance.
    pub fn bump_dist(&self) -> f64 {
        self.bump_dist
    }

    /// Generate a list of material mappings.
    pub fn generate_mat_maps(&self) -> Vec<(&str, Array3<f64>)> {
        let mut maps = Vec::with_capacity(self.materials.len());

        let mats = self.grid.cells().map(|cell| cell.mat().name());

        for mat in &self.materials {
            let name = mat.name();
            maps.push((name, mats.map(|n| if *n == name { 1.0 } else { 0.0 })));
        }

        maps
    }

    /// Generate a list of species concentration mappings.
    pub fn generate_conc_maps(&self) -> Vec<(&str, Array3<f64>)> {
        let mut maps = Vec::with_capacity(self.species.len());

        for (index, spec) in self.species.iter().enumerate() {
            let name = spec.name();
            maps.push((
                name,
                self.grid.cells().map(|cell| cell.state().concs()[index]),
            ));
        }

        maps
    }
}
