//! Verse implementation.

use crate::{
    access,
    chem::Reaction,
    dom::{load_set, load_surfs, Name, Set},
    uni::{Interface, Material, Verse as UniVerse},
};
use attr::json;
use std::{collections::BTreeMap, path::Path};

/// Verse construction form.
#[json]
pub struct Verse {
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
        let surfs = load_surfs(
            &in_dir.join("surfs"),
            &surf_list,
            "json",
            &in_dir.join("meshes"),
            "obj",
        );

        let inters = Set::new(self.inters);

        UniVerse::new(mats, surfs, inters)
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
}
