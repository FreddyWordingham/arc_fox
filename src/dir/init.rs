//! Directory initialisation functions.

use super::{input, output};
use contracts::{post, pre};
use std::{
    env::{current_dir, set_current_dir},
    fs::create_dir_all,
    path::PathBuf,
};

/// Initialise the current working directory for input.
/// Sets the current working directory to the arc internal working input folder.
#[pre(!sub_dir.is_empty())]
#[post(ret.is_dir())]
pub fn input_dir(sub_dir: &str) -> PathBuf {
    let cwd = input().join(sub_dir);
    set_current_dir(cwd).expect("Unable to set the current working directory!");
    current_dir().expect("Unable to determine the current working directory.")
}

/// Create a path to the output directory.
/// Create the directories if they do not exist.
#[post(ret.is_dir())]
pub fn output_dir(sub_dir: &str) -> PathBuf {
    let out = output().join(sub_dir);
    create_dir_all(&out).expect("Could not create output directory.");
    out
}
