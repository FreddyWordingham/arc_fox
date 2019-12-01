//! Reactions alias.

use crate::sci::chem::{Reaction, ReactionBuilder, Species};
use log::info;
use std::collections::HashMap;

/// Build the reaction list.
pub fn build(build_map: HashMap<String, ReactionBuilder>, species: &[Species]) -> Vec<Reaction> {
    let mut list = Vec::with_capacity(build_map.len());

    for (name, builder) in build_map {
        info!("Building reaction: {}", name);
        list.push(Reaction::build(name, builder, species));
    }

    list
}
