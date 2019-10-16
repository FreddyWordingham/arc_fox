//! Boundary model input.

use crate::{
    dir::meshes, file::Loadable, geom::Triangle, phy::Material, world::Boundary as wBoundary,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Boundary setup information.
#[derive(Debug, Deserialize, Serialize)]
pub struct Boundary {
    /// Mesh boundary.
    mesh: String,
    /// Inside material.
    inside: String,
    /// Outside material.
    outside: String,
}

impl Boundary {
    /// Construct a new instance.
    pub fn new(mesh: String, inside: String, outside: String) -> Self {
        Self {
            mesh,
            inside,
            outside,
        }
    }

    /// Manifest into a completed structure.
    pub fn manifest<'a>(&self, mat_map: &'a HashMap<String, Material>) -> wBoundary<'a> {
        let tris = Vec::<Triangle>::load(&meshes().join(format!("{}.obj", self.mesh)));
        let inside = &mat_map[&self.inside];
        let outside = &mat_map[&self.outside];

        wBoundary::new(tris, inside, outside)
    }

    /// Reference the mesh string.
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
}
