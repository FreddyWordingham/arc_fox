//! Set implementation.

use crate::{access, file::Load};
use log::info;
use std::{collections::BTreeMap, path::Path};

/// Set mapping.
pub struct Set<T> {
    /// Internal map.
    map: BTreeMap<String, T>,
}

impl<T: Load> Set<T> {
    access!(map, BTreeMap<String, T>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(map: BTreeMap<String, T>) -> Self {
        Self { map }
    }

    /// Construct a new instance by loading reaction files.
    #[inline]
    #[must_use]
    pub fn load(dir: &Path, names: &[String], ext: &str) -> Self {
        let mut map = BTreeMap::new();

        for name in names {
            let path = dir.join(format!("{}.{}", name, ext));
            info!("Loading: {}", path.display());

            map.insert(name.to_string(), T::load(&path));
        }

        Self::new(map)
    }
}
