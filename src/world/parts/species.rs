//! Species alias.

use crate::sci::chem::{Species, SpeciesBuilder};
use log::info;
use std::collections::HashMap;

/// Build the species list.
pub fn build(build_map: HashMap<String, SpeciesBuilder>) -> Vec<Species> {
    let mut list = Vec::with_capacity(build_map.len());

    for (name, builder) in build_map {
        info!("Building species: {}", name);
        list.push(Species::build(name, &builder));
    }

    list
}
