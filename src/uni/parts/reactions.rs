//! Reactions type alias.

use crate::{access, file::io::Load, sci::chem::Reaction};
use log::info;
use std::{collections::BTreeMap, ops::Index, path::Path};

/// Reaction mapping.
pub struct Reactions {
    /// Internal map ping.
    map: BTreeMap<String, Reaction>,
}

impl Reactions {
    access!(map, BTreeMap<String, Reaction>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(map: BTreeMap<String, Reaction>) -> Self {
        Self { map }
    }

    /// Construct a new instance by loading reaction files.
    #[inline]
    #[must_use]
    pub fn load(dir: &Path, names: &[String]) -> Self {
        let mut map = BTreeMap::new();

        for name in names {
            let path = dir.join(format!("{}.json", name));
            info!("Loading reaction: {}", path.display());

            map.insert(name.to_string(), Reaction::load(&path));
        }

        Self::new(map)
    }
}

impl Index<&str> for Reactions {
    type Output = Reaction;

    #[inline]
    #[must_use]
    fn index(&self, st: &str) -> &Self::Output {
        self.map
            .get(st)
            .expect("Did not find id entry within reaction map.")
    }
}
