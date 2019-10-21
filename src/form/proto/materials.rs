//! Domain proto-structure.

use crate::{phy::Material, file::Loadable};
use log::info;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

/// Proto-domain structure used to manifest domain structures.
#[derive(Debug, Deserialize, Serialize)]
pub struct Materials {
    /// List of material names.
    names: Vec<String>,
}

impl Materials {
    /// Construct a new instance.
    pub fn new(names: Vec<String>) -> Self {
        Self { names }
    }

    /// Manifest the proto-domain into a full domain structure.
    pub fn manifest(&self, mat_dir: &Path) -> HashMap<String, Material> {
        let mut mat_map = HashMap::with_capacity(self.names.len());

        for name in self.names.iter() {
            info!("Loading {} material...", name);
            mat_map.insert(name.clone(), Material::load(&mat_dir.join(format!("{}.json", name))));
        }

        mat_map
    }
}
