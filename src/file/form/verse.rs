//! Verse implementation.

use crate::{
    access,
    chem::{Reaction, Species},
    dom::{load_set, load_surfs, Name, Regular, Set},
    file::Grid as FileGrid,
    uni::{Interface, Material, Verse as UniVerse},
};
use attr::json;
use std::{collections::BTreeMap, path::Path};

/// Verse construction form.
#[json]
pub struct Verse {
    /// Grid information.
    grid: FileGrid,
    /// List of interfaces.
    inters: BTreeMap<Name, Interface>,
    /// List of reactions.
    reacts: BTreeMap<Name, Reaction>,
}

impl Verse {
    access!(inters, BTreeMap<Name, Interface>);

    /// Form a new instance.
    #[inline]
    #[must_use]
    pub fn form(self, in_dir: &Path) -> UniVerse {
        let mat_list = self.mat_list();
        let mats = load_set::<Material>(&in_dir.join("mats"), &mat_list, "json");

        let surf_list = self.surf_list();
        let meshes = load_surfs(
            &in_dir.join("surfs"),
            &surf_list,
            "json",
            &in_dir.join("meshes"),
            "obj",
        );

        let spec_list = self.spec_list();
        let specs = load_set::<Species>(&in_dir.join("specs"), &spec_list, "json");

        let inters = Set::new(self.inters);
        let reacts = Set::new(self.reacts);

        let grid = Regular::new(
            crate::geom::Aabb::new(*self.grid.mins(), *self.grid.maxs()),
            *self.grid.res(),
            &inters,
            &meshes,
        );

        UniVerse::new(mats, meshes, inters, specs, reacts, grid)
    }

    /// Create a list of all used materials.
    #[inline]
    #[must_use]
    pub fn mat_list(&self) -> Vec<Name> {
        self.inters
            .values()
            .flat_map(|inter| vec![inter.in_mat().clone(), inter.out_mat().clone()])
            .collect()
    }

    /// Create a list of all used surfaces.
    #[inline]
    #[must_use]
    pub fn surf_list(&self) -> Vec<Name> {
        self.inters
            .values()
            .map(|inter| inter.surf().clone())
            .collect()
    }

    /// Create a list of all used species.
    #[inline]
    #[must_use]
    pub fn spec_list(&self) -> Vec<Name> {
        self.reacts
            .values()
            .flat_map(Reaction::req_species)
            .collect()
    }
}
