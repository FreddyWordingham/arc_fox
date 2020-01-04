//! Savable trait.

use serde::Serialize;
use serde_json::to_string;
use std::{fmt::Debug, fs::write, path::Path};

/// Types implementing this trait can be saved to file.
pub trait Save: Debug {
    /// Serialise the type to a given file.
    fn save(&self, path: &Path);
}

/// Serialise the type in json format.
pub fn as_json<T: Serialize>(instance: &T, path: &Path) {
    write(
        path,
        to_string(instance).expect("Unable to serialise object."),
    )
    .expect("Unable to write json file.");
}
