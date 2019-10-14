//! Directory path constants.

use std::{env::var, path::PathBuf};

/// Create a path to the arc resources directory.
pub fn resources() -> PathBuf {
    let arc_dir = var("ARC_DIR").unwrap();

    PathBuf::from(arc_dir).join("res/")
}
