//! Common start-up operations.

use log::error;
use std::{env::args, env::set_current_dir, fs::create_dir_all, path::PathBuf};

/// Get the command line arguments.
pub fn get_args(hints: Vec<String>) -> Vec<String> {
    let args: Vec<String> = args().collect();

    if args.len() != (hints.len() + 1) {
        error!("Required call:\n{} {}", args[0], hints.join(" "));
        panic!("Invalid command line arguments!");
    }

    args
}

/// Get the current working directory.
/// This also sets the current working directory to the arc internal working folder.
pub fn get_cwd() -> PathBuf {
    set_current_dir(path).expect("Unable to set the current working directory!");
}

/// Create an output directory.
pub fn create_output_dir() -> PathBuf {
    create_dir_all("./out").unwrap()
}
