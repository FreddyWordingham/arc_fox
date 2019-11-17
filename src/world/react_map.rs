//! Reaction-map alias.

use super::MolMap;
use crate::chem::{ProtoReaction, Reaction};
use contracts::pre;
use log::info;
use std::{collections::HashMap, path::Path};

/// Reaction-map alias.
pub type ReactMap = HashMap<String, Reaction>;

/// Construct a reaction-map from a hashmap of proto-reactions.
#[pre(react_dir.is_dir())]
#[pre(!proto_react_map.is_empty())]
#[post(!ret.is_empty())]
pub fn new_react_map(
    react_dir: &Path,
    proto_react_map: &HashMap<String, ProtoReaction>,
    mol_map: &MolMap,
) -> ReactMap {
    info!("Constructing the reaction map...");

    let mut react_map = ReactMap::with_capacity(proto_react_map.len());
    for (id, proto_react) in proto_react_map.iter() {
        info!("Loading reaction: {}", id);
        react_map.insert(id.to_string(), Reaction::build(mol_map, proto_react));
    }

    info!("Loaded {} total reactions.", react_map.len());

    react_map
}
