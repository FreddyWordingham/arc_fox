//! Loadable from file trait.

use contracts::pre;
use serde::Deserialize;
use serde_json::from_reader;
use std::{fs::File, io::BufReader, path::Path};

/// Types implementing this trait can be loaded from a file on disk.
pub trait Loadable {
    /// Deserialise the type from a given file.
    fn load(path: &Path) -> Self;
}

impl<T> Loadable for T
where
    for<'a> T: Deserialize<'a>,
{
    #[pre(path.is_file(), "Invalid file to load resource from.")]
    fn load(path: &Path) -> Self {
        let file = File::open(path).expect("Unable to open file!");
        from_reader(BufReader::new(file)).expect("Unable to parse object from json file!")
    }
}
