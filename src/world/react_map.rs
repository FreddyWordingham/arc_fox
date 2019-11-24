//! Reaction-map alias.

use super::MolMap;
use crate::{
    chem::{ProtoReaction, Reaction},
    util::progress::bar,
};
use std::collections::HashMap;

/// Reaction-map alias.
pub type ReactMap = HashMap<String, Reaction>;

/// Construct a reaction-map from a hashmap of proto-reactions.
pub fn new_react_map(
    proto_react_map: &HashMap<String, ProtoReaction>,
    mol_map: &MolMap,
) -> ReactMap {
    let pb = bar("Loading reactions", proto_react_map.len() as u64);

    let mut react_map = ReactMap::with_capacity(proto_react_map.len());
    for (id, proto_react) in proto_react_map.iter() {
        pb.inc(1);

        react_map.insert(id.to_string(), Reaction::build(mol_map, proto_react));
    }

    pb.finish_with_message("Regions constructed.");

    react_map
}
