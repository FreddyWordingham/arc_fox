//! Initialisation functions.

use crate::util::{dirs::install::root, info::exec};
use std::{
    env::{current_dir, set_current_dir},
    fs::create_dir_all,
    path::PathBuf,
};

/// Set and get the input and output directories.
/// Returned pair is (input, output).
pub fn io_dirs(input: Option<PathBuf>, output: Option<PathBuf>) -> (PathBuf, PathBuf) {
    let in_dir = if let Some(input) = input {
        input
    } else {
        root().join("input").join(exec::name())
    };

    let out_dir = if let Some(output) = output {
        output
    } else {
        root().join("output").join(exec::name())
    };

    (input_dir(in_dir), output_dir(out_dir))
}

/// Initialise the current working directory.
fn input_dir(dir: PathBuf) -> PathBuf {
    set_current_dir(dir).expect("Unable to set the current working directory.");
    current_dir().expect("Unable to determine the current working directory.")
}

/// Create an output directory.
fn output_dir(dir: PathBuf) -> PathBuf {
    create_dir_all(&dir).expect("Unable to create output directory.");
    dir.to_path_buf()
}
