//! Interfaces set functions.

use crate::{
    dom::{Name, Set},
    geom::{Aabb, Mesh, Ray, Trace},
    uni::Interface,
};

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
        if let Some((dist, inside)) = meshes
            .map()
            .get(interface.surf())
            .expect("Invalid mesh name.")
            .dist_inside(ray)
        {
            if nearest.is_none()
                || dist
                    < nearest
                        .expect("Something went wrong that shouldn't have.")
                        .1
            {
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

        return Some(name.clone());
    }

    None
}
