//! Initialisation functions.

use crate::{dir::arc, util::bin_name};
use contracts::{post, pre};
use std::{
    env::{current_dir, set_current_dir},
    fs::create_dir_all,
    path::PathBuf,
};

/// Quickly import the command line arguments as their requested type.
#[macro_export]
macro_rules! args {
    ($($name:ident : $type:ty), +) => {
        let args: Vec<String> = std::env::args().collect();
        let mut args_iter = args.iter();
        $(
            let $name = (*args_iter.next().expect(
                &format!("Command line argument <{}> missing.", stringify!($name)))).parse::<$type>().expect(
                &format!("Unable to parse <{}> into {}.", stringify!($name), stringify!($type))
            );
        )*
    };
}

/// Set and get the input and output directories.
/// Returned pair is (input, output).
#[post(ret.0.is_dir(), "Input directory path is invalid.")]
#[post(ret.1.is_dir(), "Output directory path is invalid.")]
pub fn io_dirs(input: Option<PathBuf>, output: Option<PathBuf>) -> (PathBuf, PathBuf) {
    let in_dir = if input.is_some() {
        input.unwrap()
    } else {
        arc().join("input").join(bin_name())
    };
    let out_dir = if output.is_some() {
        output.unwrap()
    } else {
        arc().join("output").join(bin_name())
    };
    (input_dir(&in_dir), output_dir(&out_dir))
}

/// Initialise the current working directory.
#[pre(dir.is_dir(), "Invalid input directory path requested.")]
#[post(ret.is_dir(), "Input directory path is invalid.")]
fn input_dir(dir: &PathBuf) -> PathBuf {
    set_current_dir(dir).expect("Unable to set the current working directory.");
    current_dir().expect("Unable to get the determine the current working directory.")
}

/// Create an output directory.
#[post(ret.is_dir(), "Output directory path is invalid.")]
fn output_dir(dir: &PathBuf) -> PathBuf {
    create_dir_all(&dir).expect("Could not create output directory.");
    dir.to_path_buf()
}
