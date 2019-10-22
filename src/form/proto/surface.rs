//! Surface proto-structure.

use crate::{file::Loadable, geom::Triangle, phy::Material, phy::Surface as NeoSurface};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

/// Proto-surface structure used to manifest surface structures.
#[derive(Debug, Deserialize, Serialize)]
pub struct Surface {
    /// Mesh file name.
    mesh: String,
    /// Inside material name.
    inside: String,
    /// Outside material name.
    outside: String,
}

impl Surface {
    /// Construct a new instance.
    pub fn new(mesh: String, inside: String, outside: String) -> Self {
        Self {
            mesh,
            inside,
            outside,
        }
    }

    /// Reference the mesh file string.
    pub fn mesh(&self) -> &String {
        &self.mesh
    }

    /// Reference the inside string.
    pub fn inside(&self) -> &String {
        &self.inside
    }

    /// Reference the outside string.
    pub fn outside(&self) -> &String {
        &self.outside
    }

    /// Manifest the proto-surface into a full surface structure.
    pub fn manifest<'a>(
        &self,
        mesh_dir: &Path,
        mat_map: &'a HashMap<String, Material>,
    ) -> NeoSurface<'a> {
        let tris = Vec::<Triangle>::load(&mesh_dir.join(format!("{}.obj", self.mesh)));

        let inside = &mat_map[&self.inside];
        let outside = &mat_map[&self.outside];

        NeoSurface::new(tris, inside, outside)
    }
}
