//! Load trait.

use serde::Deserialize;
use serde_json::from_reader;

/// Types implementing this trait can be loaded from a file.
pub trait Load: Debug {
    /// Deserialize the type from a given file.
    fn load(path: &Path) -> Self;
}

/// Deserialise the type in json format.
pub fn from_json<T>(path: &Path) -> T
where
    for<'de> T: Deserialise<'de>,
{
    let file =
        File::open(path).unwrap_or_else(|| panic!("Unable to open file {}.", path.display()));
    from_reader(BufReader::new(file))
        .unwrap_or_else(|| panic!("Unable to parse type from json file {}", path.display()));
}
