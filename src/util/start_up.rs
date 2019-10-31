//! Common start-up operations.

use log::error;
use std::{
    env::{args, current_dir, set_current_dir, var},
    fs::create_dir_all,
    path::{Path, PathBuf},
};

/// Get the command line arguments.
pub fn get_args(hints: Vec<String>) -> Vec<String> {
    let args: Vec<String> = args().collect();

    if args.len() != (hints.len() + 1) {
        error!("Required call:\n{} {}", args[0], hints.join(" "));
        panic!("Invalid command line arguments!");
    }

    args
}

/// Initialise the current working directory.
/// Sets the current working directory to the arc internal working folder.
pub fn init_cwd(sub_dir: &str) -> PathBuf {
    let arc_env_var = var("ARC_DIR").expect("Environment variable ARC_DIR must be set not set.");
    let arc_dir = Path::new(&arc_env_var);

    let cwd = arc_dir.join("cwd").join(sub_dir);
    println!("Cwd: {}", cwd.display());

    set_current_dir(cwd).expect("Unable to set the current working directory!");

    current_dir().expect("Unable to get the determine the current working directory.")
}

/// Create an output directory.
pub fn init_out() -> PathBuf {
    let out = current_dir()
        .expect("Unable to get the determine the current working directory.")
        .join("out");

    create_dir_all(&out).expect("Could not create output directory.");

    out
}
