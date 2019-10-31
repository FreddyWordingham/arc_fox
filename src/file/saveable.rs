//! Saveable to file trait.

use serde::Serialize;
use serde_json::to_string;
use std::{fs::write, path::Path};

/// Types implementing this trait can be saved to a file on disk.
pub trait Saveable {
    /// Serialise the type to a given file.
    fn save(&self, path: &Path);
}

impl<T: Serialize> Saveable for T {
    fn save(&self, path: &Path) {
        write(path, to_string(self).expect("Unable to serialise object!"))
            .expect("Unable to write json file!")
    }
}
