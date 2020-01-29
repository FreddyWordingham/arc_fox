//! Interface set functions.

use crate::file::Interface;
use std::collections::BTreeMap;

/// Get a list of all materials used by the interfaces.
#[inline]
#[must_use]
pub fn filter_materials(interfaces: &BTreeMap<String, Interface>) -> Vec<String> {
    interfaces
        .values()
        .flat_map(|inter| vec![inter.in_mat().to_string(), inter.out_mat().to_string()])
        .collect()
}
