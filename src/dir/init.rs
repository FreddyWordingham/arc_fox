//! Directory initialisation functions.

use std::{
    env::{current_dir, set_current_dir, var},
    fs::create_dir_all,
    path::{Path, PathBuf},
};

/// Initialise the current working directory.
/// Sets the current working directory to the arc internal working folder.
pub fn cwd(sub_dir: &str) -> PathBuf {
    let arc_env_var = var("ARC_DIR").expect("Environment variable ARC_DIR must be set not set.");
    let arc_dir = Path::new(&arc_env_var);

    let cwd = arc_dir.join("cwd").join(sub_dir);
    println!("Cwd: {}", cwd.display());

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
