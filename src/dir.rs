//! Directory setup and locations.

pub mod res;

pub use self::res::*;

use contracts::post;
use std::{
    env::var,
    path::{Path, PathBuf},
};

/// Get the arc directory path from the environment variable.
/// Environment variable must be set.
#[post(ret.is_dir(), "Invalid ARC_DIR environment variable.")]
pub fn arc() -> PathBuf {
    Path::new(&var("ARC_DIR").expect("Environment variable ARC_DIR is not set.")).to_path_buf()
}
