//! Input form structures.

pub mod manifest;

pub use self::manifest::*;

use crate::file::Loadable;
use log::info;
use std::{fmt::Debug, path::Path};

/// Load a manifest type from a json file.
pub fn load<T: Debug + Loadable>(path: &Path) -> T {
    info!("Loading manifest file: {}", path.display());
    let man = T::load(path);
    info!("Manifest loaded:\n{:#?}", man);

    man
}
