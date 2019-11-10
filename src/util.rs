//! Utility functions and structures.

pub mod sort_label;

pub use self::sort_label::*;

use std::{env::args, path::Path};

/// Get the binary name.
pub fn bin_name() -> String {
    let args: Vec<String> = args().collect();
    Path::new(&args[0])
        .file_name()
        .expect("Unable to determine binary name.")
        .to_str()
        .expect("Unable to convert binary name into str.")
        .to_string()
}
