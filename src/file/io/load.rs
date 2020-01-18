//! Loadable trait.

use json5;
use serde::Deserialize;
use serde_json::from_reader;
use std::{
    collections::BTreeMap,
    fmt::Debug,
    fs::{read_to_string, File},
    io::BufReader,
    path::Path,
};

/// Types implementing this trait can be loaded from a file.
pub trait Load: Debug {
    /// Deserialize the type from a given file.
    fn load(path: &Path) -> Self;
}

/// Deserialise the type in json format.
#[inline]
#[must_use]
pub fn from_json<T>(path: &Path) -> T
where
    for<'de> T: Deserialize<'de>,
{
    json5::from_str(&read_to_string(path).expect("Unable to read file."))
        .expect("Unable to parse json file.")
}

/// Load a map of instances.
#[inline]
#[must_use]
pub fn map<T: Load>(dir: &Path, names: &[String]) -> BTreeMap<String, T> {
    let mut map = BTreeMap::new();

    for name in names {
        let path = dir.join(name).with_extension("json");
        map.insert(name.to_string(), T::load(&path));
    }

    map
}
