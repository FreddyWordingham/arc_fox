//! Interfaces alias.

use crate::{
    sci::math::shape::Mesh,
    world::mat::{Interface, InterfaceBuilder, Material},
};
use log::info;
use std::collections::HashMap;

/// Build the interfaces list.
pub fn build<'a>(
    build_map: HashMap<String, InterfaceBuilder>,
    meshes: &HashMap<String, Mesh>,
    materials: &'a [Material],
) -> Vec<Interface<'a>> {
    let mut list = Vec::with_capacity(build_map.len());

    for (name, builder) in build_map {
        info!("Building interface: {}", name);
        list.push(Interface::build(name, builder, meshes, materials));
    }

    list
}
