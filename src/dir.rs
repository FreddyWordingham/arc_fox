//! Directory setup and locations.

use std::{
    env::var,
    path::{Path, PathBuf},
};

pub mod init;
pub mod res;

pub use self::init::*;
pub use self::res::*;

/// Get the arc directory path from the environment variable.
/// Environment variable must be set.
pub fn arc() -> PathBuf {
    Path::new(&var("ARC_DIR").expect("Environment variable ARC_DIR must be set not set."))
        .to_path_buf()
}
