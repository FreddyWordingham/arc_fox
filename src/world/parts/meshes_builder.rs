//! Materials-builder functions.

use crate::{file::io::Load, sci::math::shape::Mesh, world::mat::InterfaceBuilder};
use contracts::pre;
use log::info;
use std::{collections::HashMap, path::Path};

/// Load a map of mesh-builders.
#[pre(dir.is_dir())]
pub fn load(dir: &Path, interfaces: &HashMap<String, InterfaceBuilder>) -> HashMap<String, Mesh> {
    let mut names = Vec::new();

    for interface in interfaces.values() {
        names.push(interface.mesh.name.clone());
    }

    names.sort();
    names.dedup();

    let mut meshes = HashMap::with_capacity(names.len());
    for name in names {
        let path = dir.join(format!("{}.obj", name));
        info!("Loading mesh: {}", name);
        let tris = Vec::load(&path);
        meshes.insert(name, Mesh::new(tris));
    }

    meshes
}
