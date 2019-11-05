//! Universal information structure.

#![allow(unused_variables)]

use super::{load_ent_map, load_mat_map, EntMap, MatMap};
use crate::{
    data::Archive,
    dir::res::mats,
    dom::{Aabb, Grid},
    index::Layout,
    proto::Entity as ProtoEntity,
};
use self_ref::self_referencing;
use std::sync::Arc;

/// Structure containing all simulation information.
#[derive(Debug)]
pub struct Universe<'a> {
    /// Data archive.
    pub archive: Archive,
    /// Map of all materials.
    pub mat_map: MatMap,
    /// Map of all entities.
    pub ent_map: EntMap<'a>,
    /// Grid of cells.
    pub grid: Grid<'a>,
}

impl<'a> Universe<'a> {
    /// Construct a new instance.
    pub fn new(layout: Layout, aabb: Aabb, ents: Vec<ProtoEntity>) -> Self {
        let archive = Archive::new(layout.clone());

        let mut mat_names = Vec::new();
        for ent in ents.iter() {
            mat_names.push(ent.in_mat);
            mat_names.push(ent.out_mat);
        }
        mat_names.sort();
        mat_names.dedup();

        Arc::try_unwrap(self_referencing!(Universe, {
            archive = archive;
            mat_map = load_mat_map(&mats(), &mat_names);
            ent_map = load_ent_map(ents, &mat_map);
            grid = Grid::new(layout, aabb, &ent_map);
        }))
        .unwrap()
    }
}
