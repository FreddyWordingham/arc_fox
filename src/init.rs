//! Initialisation functions.

use crate::{dir::arc, util::bin_name};
use std::path::PathBuf;

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
pub fn io(input: Option<PathBuf>, output: Option<PathBuf>) -> (PathBuf, PathBuf) {
    let in_dir = if input.is_some() {
        input.unwrap()
    } else {
        arc().join("input").join(bin_name())
    };
}

// /// Initialise the current working directory.
// /// Sets the current working directory to the arc internal working folder.
// fn input(sub_dir: &str) -> PathBuf {
//     let cwd = arc().join("cwd").join(sub_dir);

//     set_current_dir(cwd).expect("Unable to set the current working directory!");

//     current_dir().expect("Unable to get the determine the current working directory.")
// }

// /// Create an output directory.
// fn output() -> PathBuf {
//     let out = current_dir()
//         .expect("Unable to get the determine the current working directory.")
//         .join("out");

//     create_dir_all(&out).expect("Could not create output directory.");

//     out
// }
