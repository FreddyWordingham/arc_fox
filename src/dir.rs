//! Directory setup and locations.

use contracts::post;
use std::{
    env::var,
    path::{Path, PathBuf},
};

/// Get the arc directory path from the environment variable.
/// Environment variable must be set.
#[post(ret.is_dir())]
pub fn arc() -> PathBuf {
    Path::new(&var("ARC_DIR").expect("ARC_DIR environment variable is not set.")).to_path_buf()
}
