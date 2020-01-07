//! Loadable trait.

use serde::Deserialize;
use serde_json::from_reader;
use std::{fmt::Debug, fs::File, io::BufReader, path::Path};

/// Types implementing this trait can be loaded from a file.
pub trait Load: Debug {
    /// Deserialize the type from a given file.
    fn load(path: &Path) -> Self;
}

/// Deserialise the type in json format.
#[inline]
pub fn from_json<T>(path: &Path) -> T
where
    for<'de> T: Deserialize<'de>,
{
    let file =
        File::open(path).unwrap_or_else(|_| panic!("Unable to open file {}.", path.display()));
    from_reader(BufReader::new(file))
        .unwrap_or_else(|_| panic!("Unable to parse type from json file {}", path.display()))
}
