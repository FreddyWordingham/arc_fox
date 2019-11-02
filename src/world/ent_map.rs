//! Entity map alias.

use super::Entity;
use crate::{
    geom::{fibonacci_ray_cast, Aabb, Ray, Traceable},
    phys::Material,
};
use contracts::pre;
use log::warn;
use nalgebra::{Point3, Unit, Vector3};
use std::collections::HashMap;

/// Entity map alias type.
pub type EntMap<'a> = HashMap<&'static str, Entity<'a>>;

/// Determine the observable material at a given point.
#[pre(dom.contains(point))]
pub fn mat_at_point<'a>(point: &Point3<f64>, dom: &Aabb, ent_map: &'a EntMap) -> &'a Material {
    let n: i32 = 7;
    let mut power = 2;
    loop {
        for i in -n.pow(power)..=n.pow(power) {
            let ray = Ray::new(point.clone(), fibonacci_ray_cast(i, n.pow(power)));

            let mut nearest: Option<(f64, Unit<Vector3<f64>>, &Entity)> = None;
            for (_name, ent) in ent_map {
                if ent.boundary().hit(&ray) {
                    for s in ent.surfs() {
                        if let Some((dist, norm)) = s.dist_norm(&ray) {
                            if nearest.is_none() || (dist > nearest.unwrap().0) {
                                nearest = Some((dist, norm, ent));
                            }
                        }
                    }
                }
            }

            if let Some((dist, norm, ent)) = nearest {
                if dist <= dom.dist(&ray).unwrap() {
                    if norm.dot(&ray.dir) < 0.0 {
                        return ent.out_mat();
                    }

                    return ent.in_mat();
                }
            }
        }

        if power < 4 {
            warn!(
                "Increasing ray-casting power ({} rays)",
                (2 * n.pow(power)) + 1
            );
            power += 1;
        } else {
            break;
        }
    }

    panic!("Unable to observe a material from given point.");
}
