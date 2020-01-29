//! Interface set functions.

use crate::{
    dom::Set,
    file::Interface as FileInterface,
    sim::{Interface, Material},
};
use std::{collections::BTreeMap, path::Path};

/// Get a list of all materials used by the interfaces.
#[inline]
#[must_use]
pub fn build_interfaces<'a>(
    in_dir: &Path,
    interfaces: &BTreeMap<String, FileInterface>,
    materials: &'a Set<Material>,
) -> Set<Interface<'a>> {
    let mut set = BTreeMap::new();

    for (name, interface) in interfaces.iter() {
        set.insert(
            name.to_string(),
            Interface::new(
                interface.surf().build(in_dir),
                materials.map().get(interface.in_mat()).unwrap(),
                materials.map().get(interface.out_mat()).unwrap(),
            ),
        );
    }

    Set::new(set)
}
