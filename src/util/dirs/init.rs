//! Initialisation functions.

use crate::util::{dirs::install::root, info::exec};
use contracts::{post, pre};
use std::{
    env::{current_dir, set_current_dir},
    fs::create_dir_all,
    path::PathBuf,
};

/// Set and get the input and output directories.
/// Returned pair is (input, output).
#[pre(input.is_none() || input.as_ref().unwrap().is_dir())]
#[post(ret.0.is_dir())]
#[post(ret.1.is_dir())]
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
// #[pre(dir.is_dir())]
// #[post(ret.is_dir())]
fn input_dir(dir: PathBuf) -> PathBuf {
    crate::report!(dir.display());

    set_current_dir(dir).expect("Unable to set the current working directory.");
    current_dir().expect("Unable to determine the current working directory.")
}

/// Create an output directory.
#[post(ret.is_dir())]
fn output_dir(dir: PathBuf) -> PathBuf {
    create_dir_all(&dir).expect("Unable to create output directory.");
    dir.to_path_buf()
}
