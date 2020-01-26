//! Functions for sets of interfaces.

use crate::ord::Set;
use crate::sci::phys::Interface;

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
