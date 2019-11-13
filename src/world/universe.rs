//! Universal information structure.

#![allow(unused_variables)]

use super::{load, Entity, Material};
use crate::{
    data::Archive,
    dim::Cartesian::{X, Y, Z},
    dom::Grid,
    form::Setup,
    geom::Aabb,
    index::Resolution,
};
use contracts::pre;
use log::info;
use nalgebra::{Point3, Similarity3, Vector3};
use self_ref::self_referencing;
use std::sync::Arc;

/// Structure containing all simulation information.
#[derive(Debug)]
pub struct Universe<'a> {
    /// List of materials within the simulation.
    mats: Vec<Material>,
    /// List of entities within the simulation.
    ents: Vec<Entity<'a>>,
    /// Grid of cells.
    grid: Grid<'a>,
}

impl<'a> Universe<'a> {
    /// Construct a new instance.
    #[pre(!mats.is_empty())]
    #[pre(!ents.is_empty())]
    pub fn newish(mats: Vec<Material>, ents: Vec<Entity<'a>>, grid: Grid<'a>) -> Self {
        Self { mats, ents, grid }
    }

    /// Reference the materials.
    pub fn mats(&self) -> &Vec<Material> {
        &self.mats
    }

    /// Reference the entities.
    pub fn ents(&self) -> &Vec<Entity> {
        &self.ents
    }

    /// Reference the grid.
    pub fn grid(&self) -> &Grid<'a> {
        &self.grid
    }

    /// Add an archive into the grid cells.
    pub fn add_archive(&mut self, archive: Archive) {
        info!("Updating world archive...");
        self.grid.add_archive(archive);
    }
}
