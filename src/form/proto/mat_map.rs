//! Material-map proto-structure.

use crate::{file::Loadable, phy::Material, world::MatMap as NeoMatMap};
use contracts::pre;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

/// Proto-material-map structure used to manifest material-map structures.
#[derive(Debug, Deserialize, Serialize)]
pub struct MatMap {
    /// List of material names.
    names: Vec<String>,
}

impl MatMap {
    /// Construct a new instance.
    #[pre(!names.is_empty())]
    pub fn new(names: Vec<String>) -> Self {
        Self { names }
    }

    /// Manifest the proto-material-map into a full material-map structure.
    pub fn manifest(&self, mat_dir: &Path) -> NeoMatMap {
        let mut mat_hash = HashMap::with_capacity(self.names.len());

        for name in self.names.iter() {
            mat_hash.insert(
                name.clone(),
                Material::load(&mat_dir.join(format!("{}.json", name))),
            );
        }

        NeoMatMap::new(mat_hash)
    }
}
