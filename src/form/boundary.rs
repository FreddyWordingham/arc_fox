//! Boundary model input.

use super::Transformation;
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
    /// Optional transform.
    transform: Option<Transformation>,
}

impl Boundary {
    /// Construct a new instance.
    pub fn new(
        mesh: String,
        inside: String,
        outside: String,
        transform: Option<Transformation>,
    ) -> Self {
        Self {
            mesh,
            inside,
            outside,
            transform,
        }
    }

    /// Manifest into a completed structure.
    pub fn manifest<'a>(&self, mat_map: &'a HashMap<String, Material>) -> wBoundary<'a> {
        let mut tris = Vec::<Triangle>::load(&meshes().join(format!("{}.obj", self.mesh)));

        if let Some(trans) = &self.transform {
            let trans = trans.manifest();
            for tri in tris.iter_mut() {
                tri.transform(&trans);
            }
        }

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
