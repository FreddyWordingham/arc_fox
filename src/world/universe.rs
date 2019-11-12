//! Universal information structure.

#![allow(unused_variables)]

use super::{load, Entity, Material};
use crate::{
    dim::Cartesian::{X, Y, Z},
    dir::res,
    dom::Grid,
    form::Setup,
    geom::Aabb,
    index::Resolution,
};
use contracts::pre;
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
    #[pre(!ent_info.is_empty())]
    pub fn new(
        dom: Aabb,
        res: Resolution,
        ent_info: Vec<(String, String, Option<Similarity3<f64>>, String, String)>,
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

    /// Construct a new instance from a setup form struct.
    pub fn new_from_setup(setup: Setup) -> Self {
        Universe::new(
            Aabb::new_centred(
                &Point3::origin(),
                &Vector3::new(
                    setup.half_widths[X as usize],
                    setup.half_widths[Y as usize],
                    setup.half_widths[Z as usize],
                ),
            ),
            Resolution::new(
                setup.resolution[X as usize],
                setup.resolution[Y as usize],
                setup.resolution[Z as usize],
            ),
            setup.ent_info,
        )
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
    pub fn grid(&self) -> &'a Grid {
        &self.grid
    }
}
