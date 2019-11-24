//! Load trait.

use serde::Deserialize;
use serde_json::from_reader;
use std::{fmt::Debug, fs::File, io::BufReader, path::Path};

/// Load trait implementation.
/// Types implementing this trait can be loaded from a file.
pub trait Load: Debug {
    /// Deserialise the type from a given file.
    fn load(path: &Path) -> Self;
}

/// Deserialise the object from json format.
pub fn from_json<T>(path: &Path) -> T
where
    for<'de> T: Deserialize<'de>,
{
    let file = File::open(path).expect(&format!("Unable to open file: {}.", path.display()));
    from_reader(BufReader::new(file)).expect(&format!(
        "Unable to parse object from json file: {}.",
        path.display()
    ))
}
