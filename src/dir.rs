//! Directory path constants.

use std::{env::var, path::PathBuf};

/// Create a path to the arc resources directory.
pub fn resources() -> PathBuf {
    let arc_dir = var("ARC_DIR").unwrap();

    PathBuf::from(arc_dir).join("res/")
}

/// Create a path to the arc materials directory.
pub fn materials() -> PathBuf {
    resources().join("mats/")
}
