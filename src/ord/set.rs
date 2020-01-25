//! Set storage map structure.

use crate::{access, file::io::Load, ord::Name};
use log::info;
use std::{collections::BTreeMap, ops::Index, path::Path};

/// Set mapping.
pub struct Set<T> {
    /// Internal map.
    map: BTreeMap<Name, T>,
}

impl<T: Load> Set<T> {
    access!(map, BTreeMap<Name, T>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(map: BTreeMap<Name, T>) -> Self {
        Self { map }
    }

    /// Construct a new instance by loading reaction files.
    #[inline]
    #[must_use]
    pub fn load(dir: &Path, names: &[Name]) -> Self {
        let mut map = BTreeMap::new();

        for name in names {
            let path = dir.join(format!("{}.json", name));
            info!("Loading reaction: {}", path.display());

            map.insert(name.to_string(), T::load(&path));
        }

        Self::new(map)
    }
}

impl<T> Index<&str> for Set<T> {
    type Output = T;

    #[inline]
    #[must_use]
    fn index(&self, st: &str) -> &Self::Output {
        self.map
            .get(st)
            .expect("Did not find id entry within the set.")
    }
}
