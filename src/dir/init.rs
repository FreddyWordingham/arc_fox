//! Directory initialisation functions.

use super::arc;
use std::{
    env::{current_dir, set_current_dir},
    fs::create_dir_all,
    path::PathBuf,
};

/// Initialise the current working directory.
/// Sets the current working directory to the arc internal working folder.
pub fn cwd(sub_dir: &str) -> PathBuf {
    let cwd = arc().join("cwd").join(sub_dir);

    set_current_dir(cwd).expect("Unable to set the current working directory!");

    current_dir().expect("Unable to get the determine the current working directory.")
}

/// Create an output directory.
pub fn output() -> PathBuf {
    let out = current_dir()
        .expect("Unable to get the determine the current working directory.")
        .join("out");

    create_dir_all(&out).expect("Could not create output directory.");

    out
}
