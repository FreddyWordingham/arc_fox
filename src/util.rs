//! Utility functions and structures.

use std::{env::args, path::Path};

/// Get the binary name.
pub fn bin_name() -> String {
    let args: Vec<String> = args().collect();
    Path::new(&args[0])
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}
