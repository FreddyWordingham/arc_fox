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
#[post(ret.is_dir(), "Could not find directory!")]
pub fn arc() -> PathBuf {
    Path::new(&var("ARC_DIR").expect("Environment variable ARC_DIR must be set not set."))
        .to_path_buf()
}
