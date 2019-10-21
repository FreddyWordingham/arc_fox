//! Directory proto-structure.

use crate::util::Dir as NeoDir;
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

impl Dir {
    /// Construct a new instance.
    pub fn new(
        cwd: Option<String>,
        out: String,
        res: String,
        mats: String,
        meshes: String,
    ) -> Self {
        Self {
            cwd,
            out,
            res,
            mats,
            meshes,
        }
    }

    /// Manifest the proto-dir into a full dir structure.
    pub fn manifest(&self) -> NeoDir {
        let cwd = if self.cwd.is_none() {
            current_dir().unwrap()
        } else {
            PathBuf::from(self.cwd.as_ref().unwrap())
        };

        let out = cwd.join(&self.out);

        let res_path = cwd.join(&self.res);

        NeoDir::new(
            cwd,
            out,
            res_path.join(&self.mats),
            res_path.join(&self.meshes),
        )
    }
}
