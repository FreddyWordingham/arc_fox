//! Universal information structure.

#![allow(unused_variables)]

use super::{load_ent_map, load_mat_map, EntMap, Light, MatMap};
use crate::{
    data::Archive,
    dir::res::mats,
    dom::{Aabb, Grid},
    index::Layout,
    proto::Entity as ProtoEntity,
};
use contracts::pre;
use log::info;
use rayon::prelude::*;
use self_ref::self_referencing;
use std::sync::Arc;

/// Structure containing all simulation information.
#[derive(Debug)]
pub struct Universe<'a> {
    /// Map of all materials.
    mat_map: MatMap,
    /// Map of all entities.
    ent_map: EntMap<'a>,
    /// Grid of cells.
    grid: Grid<'a>,
}

impl<'a> Universe<'a> {
    /// Construct a new instance.
    pub fn new(layout: Layout, aabb: Aabb, ents: Vec<ProtoEntity>) -> Self {
        let mut mat_names = Vec::new();
        for ent in ents.iter() {
            mat_names.push(ent.in_mat);
            mat_names.push(ent.out_mat);
        }
        mat_names.sort();
        mat_names.dedup();

        Arc::try_unwrap(self_referencing!(Universe, {
            mat_map = load_mat_map(&mats(), &mat_names);
            ent_map = load_ent_map(ents, &mat_map);
            grid = Grid::new(layout, aabb, &ent_map);
        }))
        .unwrap()
    }

    /// Reference the material map.
    pub fn mat_map(&self) -> &MatMap {
        &self.mat_map
    }

    /// Reference the entity map.
    pub fn ent_map(&self) -> &EntMap {
        &self.ent_map
    }

    /// Reference the grid.
    pub fn grid(&self) -> &Grid<'a> {
        &self.grid
    }

    /// Run a MCRT simulation.
    #[pre(num_threads > 0)]
    pub fn mcrt(&self, num_threads: usize, light: &Light) -> Archive {
        if num_threads == 1 {
            let thread_id = 0;
            return self.mcrt_thread(thread_id, light);
        }

        let thread_ids: Vec<usize> = (0..num_threads).collect();
        let mut archives: Vec<Archive> = thread_ids
            .par_iter()
            .map(|id| self.mcrt_thread(*id, light))
            .collect();

        let mut archive = archives.pop().unwrap();
        for a in archives.iter() {
            archive += a;
        }

        archive
    }

    fn mcrt_thread(&self, thread_id: usize, light: &Light) -> Archive {
        info!("Running from thread: {}", thread_id);
        Archive::new(self.grid.layout().clone())
    }
}
