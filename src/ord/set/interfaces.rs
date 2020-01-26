//! Functions for sets of interfaces.

use crate::{
    ord::{Name, Set},
    sci::{
        math::{
            geom::shape::{Aabb, Mesh},
            rt::{Ray, Trace},
        },
        phys::Interface,
    },
};

/// Get a list of all meshes required for the set of interfaces.
#[inline]
#[must_use]
pub fn req_meshes(interfaces: &Set<Interface>) -> Vec<String> {
    let mut meshes = Vec::new();

    for i in interfaces.map().values() {
        meshes.push(i.surf().clone());
    }

    meshes.sort();
    meshes.dedup();

    meshes
}

/// Get a list of all materials required for the set of interfaces.
#[inline]
#[must_use]
pub fn req_materials(interfaces: &Set<Interface>) -> Vec<String> {
    let mut materials = Vec::new();

    for i in interfaces.map().values() {
        materials.push(i.in_mat().clone());
        materials.push(i.out_mat().clone());
    }

    materials.sort();
    materials.dedup();

    materials
}

/// Determine which material, if any, would be observed with a given ray.
#[inline]
#[must_use]
pub fn observe_material(
    interfaces: &Set<Interface>,
    meshes: &Set<Mesh>,
    boundary: &Aabb,
    ray: &Ray,
) -> Option<Name> {
    let mut nearest: Option<(&Name, f64)> = None;

    for interface in interfaces.map().values() {
        if let Some((dist, inside)) = meshes[interface.surf()].dist_inside(ray) {
            if nearest.is_none() || dist < nearest.unwrap().1 {
                if inside {
                    nearest = Some((interface.in_mat(), dist));
                } else {
                    nearest = Some((interface.out_mat(), dist));
                }
            }
        }
    }

    if let Some((name, dist)) = nearest {
        let bound_dist = boundary
            .dist(ray)
            .expect("Observation ray did not observe boundary.");

        if bound_dist < dist {
            return None;
        }

        return Some(name.to_string());
    }

    None
}
