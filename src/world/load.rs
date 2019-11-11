//! World loading functions.

use super::Material;
use crate::file::Loadable;
use contracts::pre;
use log::{info, warn};
use std::path::Path;

/// Load a material list.
#[pre(dir.is_dir())]
#[pre(!names.is_empty())]
#[post(!ret.is_empty())]
pub fn mats(dir: &Path, names: Vec<String>) -> Vec<Material> {
    let mut sorted_names = names.clone();
    sorted_names.sort();
    sorted_names.dedup();

    if sorted_names.len() < names.len() {
        warn!(
            "Filtered out {} duplicate materials.",
            names.len() - sorted_names.len()
        );
    }

    let mut mats = Vec::with_capacity(sorted_names.len());
    for name in sorted_names {
        let path = dir.join(format!("{}.json", name));
        info!("Loading mat from file: {}", path.display());

        let mat = Material::load(&path);
        if mat.id() != name {
            warn!("Material id {} differs from name {}", mat.id(), name);
        }

        mats.push(mat);
    }

    mats
}
