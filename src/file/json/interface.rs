//! Interface implementation.

use crate::{
    access,
    dom::ord::Set,
    file::Mesh,
    sim::{Interface as SimInterface, Material},
};
use attr::json;
use std::path::Path;

/// Parsable interface structure.
#[json]
#[derive(Clone)]
pub struct Interface {
    /// Surface mesh.
    surf: Mesh,
    /// Inside material.
    in_mat: String,
    /// Outside material.
    out_mat: String,
}

impl Interface {
    access!(surf, Mesh);
    access!(in_mat, String);
    access!(out_mat, String);

    /// Build an interface.
    #[inline]
    #[must_use]
    pub fn build<'a>(self, in_dir: &Path, mats: &'a Set<Material>) -> SimInterface<'a> {
        SimInterface::new(
            self.surf.build(in_dir),
            mats.map()
                .get(&self.in_mat)
                .expect("Surface key does not exist in material map."),
            mats.map()
                .get(&self.out_mat)
                .expect("Surface key does not exist in material map."),
        )
    }
}
