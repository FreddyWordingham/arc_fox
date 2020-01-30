//! Set implementation.

use crate::{access, file::Load, file::Surface, geom::Mesh};
use log::info;
use std::{collections::BTreeMap, path::Path};

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

/// Construct a new instance of surfaces by loading wavefront files.
#[inline]
#[must_use]
pub fn load_surfs(
    surf_dir: &Path,
    names: &[String],
    surf_ext: &str,
    mesh_dir: &Path,
    mesh_ext: &str,
) -> Set<Mesh> {
    let mut map = BTreeMap::new();

    for name in names {
        let path = surf_dir.join(format!("{}.{}", name, surf_ext));
        info!("Loading: {}", path.display());

        let surf = Surface::load(&path);

        map.insert(name.to_string(), surf.build(mesh_dir, mesh_ext));
    }

    Set::new(map)
}
