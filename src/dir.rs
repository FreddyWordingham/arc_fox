//! Directory setup and locations.

use contracts::post;
use std::{
    env::var,
    path::{Path, PathBuf},
};

pub mod init;
pub mod res;

pub use self::init::*;
pub use self::res::*;

/// Get the arc root directory path from the environment variable.
/// Environment variable must be set.
#[post(ret.is_dir())]
pub fn root() -> PathBuf {
    Path::new(&var("ARC_DIR").expect("Environment variable ARC_DIR must be set not set."))
        .to_path_buf()
}

/// Get the resources directory path.
#[post(ret.is_dir())]
pub fn res() -> PathBuf {
    Path::new(&root().join("res")).to_path_buf()
}

/// Get the input directory path.
#[post(ret.is_dir())]
pub fn input() -> PathBuf {
    Path::new(&root().join("input")).to_path_buf()
}

/// Get the output directory path.
#[post(ret.is_dir())]
pub fn output() -> PathBuf {
    Path::new(&root().join("output")).to_path_buf()
}
