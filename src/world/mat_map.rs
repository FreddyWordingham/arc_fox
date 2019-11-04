//! Material map alias.

use crate::{file::Loadable, phys::Material};
use contracts::pre;
use log::{info, warn};
use std::{collections::HashMap, path::Path};

/// Material map alias type.
pub type MatMap = HashMap<&'static str, Material>;

#[pre(dir.is_dir())]
#[pre(!names.is_empty())]
pub fn load_mat_map(dir: &Path, names: &Vec<&'static str>) -> MatMap {
    let mut sorted_names = names.clone();
    sorted_names.sort();
    sorted_names.dedup();

    if sorted_names.len() != names.len() {
        warn!(
            "{} duplicate material names were removed.",
            names.len() - sorted_names.len()
        );
    }

    let mut mat_map = MatMap::with_capacity(names.len());
    info!("Loading materials...");
    for name in sorted_names {
        let path = dir.join(format!("{}.json", name));
        println!("\tLoading {}", path.display());

        mat_map.insert(name, Material::load(&path));
    }
    info!("{} materials loaded.", mat_map.len());

    mat_map
}
