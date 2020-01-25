//! Set storage map structure.

use crate::{
    access,
    file::io::Load,
    ord::Name,
    sci::{chem::Reaction, phys::Interface},
};
use log::info;
use std::{collections::BTreeMap, ops::Index, path::Path};

/// Set mapping.
pub struct Set<T> {
    /// Internal map.
    map: BTreeMap<Name, T>,
}

impl<T: Load> Set<T> {
    access!(map, BTreeMap<Name, T>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(map: BTreeMap<Name, T>) -> Self {
        Self { map }
    }

    /// Construct a new instance by loading reaction files.
    #[inline]
    #[must_use]
    pub fn load(dir: &Path, names: &[Name], ext: &str) -> Self {
        let mut map = BTreeMap::new();

        for name in names {
            let path = dir.join(format!("{}.{}", name, ext));
            info!("Loading: {}", path.display());

            map.insert(name.to_string(), T::load(&path));
        }

        Self::new(map)
    }
}

impl<T> Index<&str> for Set<T> {
    type Output = T;

    #[inline]
    #[must_use]
    fn index(&self, st: &str) -> &Self::Output {
        self.map
            .get(st)
            .expect("Did not find id entry within the set.")
    }
}

/// Get a list of all species required for the set of reactions.
#[inline]
#[must_use]
pub fn req_species(reactions: &Set<Reaction>) -> Vec<String> {
    let mut species = Vec::new();

    for r in reactions.map().values() {
        species.append(&mut r.req_species());
    }

    species.sort();
    species.dedup();

    species
}

/// Get a list of all meshes required for the set of interfaces.
#[inline]
#[must_use]
pub fn req_meshes(interfaces: &Set<Interface>) -> Vec<String> {
    let mut meshes = Vec::new();

    for i in interfaces.map().values() {
        meshes.push(i.surf().clone());
    }

    meshes.sort();
    meshes.dedup();

    meshes
}

/// Get a list of all materials required for the set of interfaces.
#[inline]
#[must_use]
pub fn req_materials(interfaces: &Set<Interface>) -> Vec<String> {
    let mut materials = Vec::new();

    for i in interfaces.map().values() {
        materials.push(i.in_mat().clone());
        materials.push(i.out_mat().clone());
    }

    materials.sort();
    materials.dedup();

    materials
}
