//! Universal information structure.

use super::{load, Entity, Material};
use crate::{dir::res, dom::Grid, geom::Aabb, index::Resolution};
use nalgebra::Similarity3;
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
    pub fn new(
        dom: Aabb,
        res: Resolution,
        ent_info: Vec<(&str, &str, Option<Similarity3<f64>>, &str, &str)>,
    ) -> Self {
        let mut mat_names = Vec::new();
        for (_id, _mesh, _trans, in_mat, out_mat) in ent_info.iter() {
            mat_names.push(in_mat.clone());
            mat_names.push(out_mat.clone());
        }
        mat_names.sort();
        mat_names.dedup();

        Arc::try_unwrap(self_referencing!(Universe, {
            mats = load::mats(&res::materials(), mat_names);
            ents = load::ents(&res::meshes(), ent_info, &mats);
            grid = Grid::new(dom, res, &ents);
        }))
        .expect("Could not create universe instance.")
    }
}
