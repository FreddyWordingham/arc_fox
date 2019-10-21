//! Directory path constants.

use std::{env::var, path::PathBuf};
use log::error;

/// Directory information storage.
pub struct Dir {
    /// Arc project root directory.
    arc: PathBuf,
}

impl Dir {
    /// Construct a new instance.
    pub fn new() -> Self {
        let arc = PathBuf::from(&var("ARC_DIR").unwrap());
        if !arc.is_dir() {
            error!("Unable to determine the arc directory!\nEnsure the environment variable ARC_DIR is correctly set.");
        }

        Self {
            arc,
        }
    }
}
