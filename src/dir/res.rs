//! Resources directory paths.

use super::arc;
use contracts::post;
use std::path::{Path, PathBuf};

/// Get the root resources directory path.
#[post(ret.is_dir())]
pub fn root() -> PathBuf {
    Path::new(&arc().join("res")).to_path_buf()
}

/// Get the materials resources sub-directory path.
#[post(ret.is_dir())]
pub fn mats() -> PathBuf {
    root().join("mats")
}

/// Get the mesh resources sub-directory path.
#[post(ret.is_dir())]
pub fn meshes() -> PathBuf {
    root().join("meshes")
}

/// Get the species resources sub-directory path.
#[post(ret.is_dir())]
pub fn species() -> PathBuf {
    root().join("species")
}
