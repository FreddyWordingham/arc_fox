//! Universal information structure.

#![allow(unused_variables)]

use super::{find_by_id, Entity, Material};
use crate::{
    data::Archive,
    dom::Grid,
    file::Loadable,
    form::Entity as EntityForm,
    geom::{Aabb, Mesh},
    index::Resolution,
};
use contracts::pre;
use log::info;
use self_ref::self_referencing;
use std::{path::Path, sync::Arc};

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
    #[pre(!ent_forms.is_empty())]
    pub fn new(in_dir: &Path, dom: Aabb, res: Resolution, ent_forms: &Vec<EntityForm>) -> Self {
        Arc::try_unwrap(self_referencing!(Universe, {
            mats = load_mats(&in_dir.join("mats"), ent_forms);
            ents = load_ents(&in_dir.join("meshes"), ent_forms, &mats);
            grid = Grid::new(dom, res, &ents);
        }))
        .expect("Could not create universe instance.")
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

/// Load the materials described within the vector of entity forms.
#[pre(dir.is_dir())]
#[pre(!ents.is_empty())]
fn load_mats(dir: &Path, ents: &Vec<EntityForm>) -> Vec<Material> {
    let ids = get_mat_ids(ents);

    let mut mats = Vec::with_capacity(ids.len());
    for id in ids.iter() {
        info!("Loading material: {}", id);

        let path = dir.join(format!("{}.json", id));
        mats.push(Material::load(&path));
    }
    info!("Loaded {} materials total.", mats.len());

    mats
}

/// Parse the material ids from the vector of entity forms.
#[pre(!ents.is_empty())]
#[post(!ret.is_empty())]
fn get_mat_ids(ents: &Vec<EntityForm>) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();

    for ent in ents.iter() {
        ids.push(ent.in_mat.clone());
        ids.push(ent.out_mat.clone());
    }

    ids.sort();
    ids.dedup();

    ids
}

/// Load the entities described within the vector of entity forms.
#[pre(dir.is_dir())]
#[pre(!ent_forms.is_empty())]
fn load_ents<'a>(
    dir: &Path,
    ent_forms: &Vec<EntityForm>,
    mats: &'a Vec<Material>,
) -> Vec<Entity<'a>> {
    let mut ents = Vec::with_capacity(ent_forms.len());
    for ent in ent_forms.iter() {
        info!("Loading entity: {}", ent.id);

        let path = dir.join(format!("{}.obj", ent.mesh));
        let mut mesh = Mesh::load(&path);

        ents.push(Entity::new(
            ent.id.clone(),
            mesh,
            find_by_id(mats, &ent.in_mat),
            find_by_id(mats, &ent.out_mat),
        ))
    }
    info!("Loaded {} entities total.", ents.len());

    ents
}
