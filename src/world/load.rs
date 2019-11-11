//! World loading functions.

use super::{Entity, Material};
use crate::{
    file::Loadable,
    geom::{Mesh, Transform},
};
use contracts::pre;
use log::{info, warn};
use nalgebra::Similarity3;
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

#[pre(dir.is_dir())]
#[pre(!ent_info.is_empty())]
#[post(!ret.is_empty())]
pub fn ents<'a>(
    dir: &Path,
    ent_info: Vec<(&str, &str, Option<Similarity3<f64>>, &str, &str)>,
    mats: &Vec<Material>,
) -> Vec<Entity<'a>> {
    let mut ents = Vec::new();

    for (id, mesh, trans, in_mat_id, out_mat_id) in ent_info {
        info!("Constructing ent: {}", id);

        let path = dir.join(format!("{}.obj", mesh));
        let mut mesh = Mesh::load(&path);

        if let Some(trans) = trans {
            mesh.transform(&trans);
        }

        let in_mat = ref_mat_id(mats, in_mat_id);
        let out_mat = ref_mat_id(mats, out_mat_id);

        ents.push(Entity::new(id, mesh, in_mat, out_mat))
    }

    ents
}

pub fn ref_mat_id<'a>(mats: &'a Vec<Material>, id: &str) -> &'a Material {
    for mat in mats {
        if mat.id() == id {
            return mat;
        }
    }

    unreachable!("Material id {} missing.", id);
}
