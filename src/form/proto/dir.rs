//! Directory proto-structure.

// use crate::file::{as_json, from_json, Loadable, Saveable};
use crate::{form::Manifestable, util::Dir};
use serde::{Deserialize, Serialize};
use std::{
    env::{create_dir_all},
};

/// Proto-dir structure used to manifest dir structures.
#[derive(Debug, Deserialize, Serialize)]
pub struct Dir {
    /// Optional target current working directory.
    cwd: Option<String>,
    /// Target relative output directory.
    out: String,
    /// Main resources directory.
    res: String,
    /// Material resources sub-directory.
    mats: String,
    /// Mesh resources sub-directory.
    meshes: String,
}

impl Manifestable<Dir> for Dir {
    fn manifest(self) -> Dir {
        let cwd =
        if self.cwd.is_none() {
              current_dir().unwrap()
        } else {
            cwd.self
        }

        Dir::new(cwd, out, self.res.join(self.mats), self.res.join(meshes))
    }
}
