//! Directory proto-structure.

use crate::{form::Manifestable, util::Dir as NeoDir};
use serde::{Deserialize, Serialize};
use std::{env::current_dir, path::PathBuf};

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

impl Manifestable<NeoDir> for Dir {
    fn manifest(self) -> NeoDir {
        let cwd = if self.cwd.is_none() {
            current_dir().unwrap()
        } else {
            PathBuf::from(self.cwd.unwrap())
        };

        let out = cwd.join(self.out);

        let res_path = PathBuf::from(self.res);

        NeoDir::new(
            cwd,
            out,
            res_path.join(self.mats),
            res_path.join(self.meshes),
        )
    }
}
