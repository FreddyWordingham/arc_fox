//! Saveable to file trait and implementations.

use serde::Serialize;
use serde_json::to_string;
use std::{fs::write, path::Path};

/// Types implementing this trait can be saved to a file on disk.
pub trait Saveable {
    /// Serialise the type to a given file.
    fn save(&self, path: &Path);
}

/// Serialise the object in json format if it implements the Serialize trait.
pub fn as_json<T: Serialize>(obj: &T, path: &Path) {
    write(path, to_string(obj).expect("Unable to serialise object!"))
        .expect("Unable to write json file!")
}
