//! Materials alias.

use crate::{
    sci::chem::Species,
    world::mat::{Material, MaterialBuilder},
};
use log::info;
use std::collections::HashMap;

/// Build the materials list.
pub fn build(build_map: HashMap<String, MaterialBuilder>, species: &[Species]) -> Vec<Material> {
    let mut list = Vec::with_capacity(build_map.len());

    for (name, builder) in build_map {
        info!("Building material: {}", name);
        list.push(Material::build(name, builder, species));
    }

    list
}
