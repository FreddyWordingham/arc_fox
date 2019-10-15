//! Common start-up operations.

use log::info;
use std::env::{current_dir, set_current_dir};
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

/// Perform the common start up operations.
pub fn start_up(cwd: &Path, out: &Path) -> (PathBuf, PathBuf) {
    colog::init();

    set_current_dir(cwd).expect("Unable to set the current working directory!");
    let cwd = current_dir().unwrap();
    info!("Current working directory: {}", cwd.display());

    create_dir_all(out).unwrap();
    info!("Output directory: {}", out.display());

    (cwd, out.to_path_buf())
}
