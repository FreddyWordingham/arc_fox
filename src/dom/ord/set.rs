//! Set implementation.

use crate::{access, file::Load};
use log::info;
use std::{collections::BTreeMap, path::Path};

pub mod interface;
pub mod materials;

pub use self::{interface::*, materials::*};

/// Set mapping.
pub struct Set<T> {
    /// Internal map.
    map: BTreeMap<String, T>,
}

impl<T> Set<T> {
    access!(map, BTreeMap<String, T>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(map: BTreeMap<String, T>) -> Self {
        Self { map }
    }

    /// Construct a new empty set.
    #[inline]
    #[must_use]
    pub fn empty() -> Self {
        Self::new(BTreeMap::new())
    }
}

/// Construct a new instance by loading reaction files.
#[inline]
#[must_use]
pub fn load_set<T: Load>(dir: &Path, names: &[String], ext: &str) -> Set<T> {
    let mut map = BTreeMap::new();

    for name in names {
        let path = dir.join(format!("{}.{}", name, ext));
        info!("Loading: {}", path.display());

        map.insert(name.to_string(), T::load(&path));
    }

    Set::new(map)
}
