//! Interfaces alias.

use crate::{
    sci::math::{
        rt::{Ray, Trace},
        shape::{Aabb, Mesh},
    },
    world::mat::{Interface, InterfaceBuilder, Material},
};
use contracts::pre;
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

/// Determine the interface and side hit.
#[pre(boundary.contains(ray.pos()))]
pub fn dist_inside_inter<'a>(
    ray: &Ray,
    boundary: &Aabb,
    interfaces: &'a [Interface<'a>],
) -> Option<(f64, bool, &'a Interface<'a>)> {
    let mut nearest: Option<(f64, bool, &'a Interface<'a>)> = None;

    let bound_dist = boundary.dist(ray).unwrap();

    for inter in interfaces {
        if let Some(dist) = inter.mesh().aabb().dist(ray) {
            if dist >= bound_dist {
                continue;
            }

            if nearest.is_none() || dist < nearest.unwrap().0 {
                if let Some((dist, inside)) = inter.mesh().dist_inside(ray) {
                    if dist >= bound_dist {
                        continue;
                    }

                    if nearest.is_none() || dist < nearest.unwrap().0 {
                        nearest = Some((dist, inside, inter));
                    }
                }
            }
        }
    }

    nearest
}
