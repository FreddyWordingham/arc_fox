//! Surface-mapping structure.

use super::super::*;
use crate::{world::MatMap as NeoMatMap, world::SurfMap as NeoSurfMap};
use contracts::pre;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

/// Proto-surface-map structure used to manifest surface-map structures.
#[derive(Debug, Deserialize, Serialize)]
pub struct SurfMap {
    /// Proto-surfaces list.
    surfs: Vec<proto::Surface>,
}

impl SurfMap {
    /// Construct a new instance.
    #[pre(!surfs.is_empty())]
    pub fn new(surfs: Vec<proto::Surface>) -> Self {
        Self { surfs }
    }

    /// Reference the proto-surface vector.
    pub fn surfs(&self) -> &Vec<proto::Surface> {
        &self.surfs
    }

    /// Manifest the proto-surface-map into a full surface-map structure.
    pub fn manifest<'a>(&self, mesh_dir: &Path, mat_map: &'a NeoMatMap) -> NeoSurfMap<'a> {
        let mut surf_hash = HashMap::with_capacity(self.surfs.len());

        for surf in self.surfs.iter() {
            surf_hash.insert(surf.mesh().clone(), surf.manifest(mesh_dir, mat_map));
        }

        NeoSurfMap::new(surf_hash)
    }
}
