//! Material-map alias.

use crate::mat::Material;
use contracts::pre;
use log::info;
use std::{collections::HashMap, path::Path};

/// Material-map alias.
pub type MatMap = HashMap<String, Material>;

/// Construct a material-map from a list of material names.
#[pre(mat_dir.is_dir())]
#[pre(!names.is_empty())]
#[post(!ret.is_empty())]
pub fn new_mat_map(mat_dir: &Path, mut names: Vec<String>) -> MatMap {
    info!("Constructing the material map...");

    names.sort();
    names.dedup();

    let mut mat_map = MatMap::with_capacity(names.len());
    for name in names.iter() {
        info!("Loading material: {}", name);
        mat_map.insert(name.to_string(), Material::new());
    }

    info!("Loaded {} total materials.", mat_map.len());

    mat_map
}
