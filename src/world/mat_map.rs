//! Material-map alias.

use crate::{file::Load, mat::Material, util::progress::bar};
use contracts::pre;
use std::{collections::HashMap, path::Path};

/// Material-map alias.
pub type MatMap = HashMap<String, Material>;

/// Construct a material-map from a list of material ids.
#[pre(mat_dir.is_dir())]
#[pre(!ids.is_empty())]
#[post(!ret.is_empty())]
pub fn new_mat_map(mat_dir: &Path, mut ids: Vec<String>) -> MatMap {
    let pb = bar("Constructing materials", ids.len() as u64);

    ids.sort();
    ids.dedup();

    let mut mat_map = MatMap::with_capacity(ids.len());
    for id in ids.iter() {
        pb.inc(1);

        mat_map.insert(
            id.to_string(),
            Material::load(&mat_dir.join(format!("{}.json", id))),
        );
    }

    pb.finish_with_message("Materials constructed.");

    mat_map
}
