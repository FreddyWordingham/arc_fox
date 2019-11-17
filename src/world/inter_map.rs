//! Interface-map alias.

use super::MatMap;
use crate::mat::{Interface, ProtoInterface};
use contracts::pre;
use log::info;
use std::{collections::HashMap, path::Path};

/// Interface-map alias.
pub type InterMap<'a> = HashMap<String, Interface<'a>>;

/// Construct a interface-map from a hashmap of proto-interfaces.
#[pre(mesh_dir.is_dir())]
#[pre(!proto_inter_map.is_empty())]
#[post(!ret.is_empty())]
pub fn new_inter_map<'a>(
    mesh_dir: &Path,
    proto_inter_map: &HashMap<String, ProtoInterface>,
    mat_map: &'a MatMap,
) -> InterMap<'a> {
    info!("Constructing the interface map...");

    let mut inter_map = InterMap::with_capacity(proto_inter_map.len());
    for (id, proto_inter) in proto_inter_map.iter() {
        info!("Loading interface: {}", id);
        inter_map.insert(
            id.to_string(),
            Interface::build(mesh_dir, proto_inter, mat_map),
        );
    }

    info!("Loaded {} total interfaces.", inter_map.len());

    inter_map
}
