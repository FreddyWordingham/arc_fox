//! Materials-builder functions.

use crate::world::{
    mat::{InterfaceBuilder, MaterialBuilder},
    parts::load_map,
};
use contracts::pre;
use std::{collections::HashMap, path::Path};

/// Load a map of material-builders.
#[pre(dir.is_dir())]
pub fn load(
    dir: &Path,
    interfaces: &HashMap<String, InterfaceBuilder>,
) -> HashMap<String, MaterialBuilder> {
    let mut names = Vec::new();

    for interface in interfaces.values() {
        names.push(interface.in_mat.clone());
        names.push(interface.out_mat.clone());
    }

    names.sort();
    names.dedup();

    load_map(dir, &names, "material")
}
