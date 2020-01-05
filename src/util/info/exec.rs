//! Executable information.

use std::{env::args, path::Path};

/// Determine the name of the executable.
pub fn name() -> String {
    let args: Vec<String> = args().collect();
    Path::new(&args[0])
        .file_name()
        .expect("Unable to determine program name.")
        .to_str()
        .expect("Unable to convert program name into str.")
        .to_string()
}
