//! Loadable from file trait.

use serde::Deserialize;
use serde_json::from_reader;
use std::{fs::File, io::BufReader, path::Path};

/// Types implementing this trait can be loaded from a file on disk.
pub trait Loadable {
    /// Deserialise the type from a given file.
    fn load(path: &Path) -> Self;
}

/// Deserialise the object in json format if it implements the Deserialize trait.
pub fn from_json<T>(path: &Path) -> T
where
    for<'de> T: Deserialize<'de>,
{
    let file = File::open(path).expect("Unable to open file.");
    from_reader(BufReader::new(file)).expect("Unable to parse object from json file.")
}
