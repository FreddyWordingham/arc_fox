//! Entity map alias.

use super::Entity;
use crate::{
    dom::Aabb,
    geom::Shape,
    phys::Material,
    rt::{fibonacci_spiral, Ray, Traceable},
};
use contracts::pre;
use log::{info, warn};
use nalgebra::Point3;
use std::collections::HashMap;

/// Entity map alias type.
pub type EntMap<'a> = HashMap<String, Entity<'a>>;

#[pre(!list.is_empty())]
#[post(!ret.is_empty())]
pub fn load_ent_map<'a>(list: Vec<(String, Shape, &'a Material, &'a Material)>) -> EntMap<'a> {
    let mut ent_map = EntMap::with_capacity(list.len());

    info!("Constructing entities...");
    for (name, shape, in_mat, out_mat) in list {
        println!("\tConstructing {}", name);

        ent_map.insert(name, Entity::new(shape, in_mat, out_mat));
    }
    info!("{} entities constructed.", ent_map.len());

    ent_map
}

#[pre(aabb.contains(p))]
pub fn mat_at_point<'a>(p: &Point3<f64>, aabb: &Aabb, ent_map: &'a EntMap) -> &'a Material {
    let n: i32 = 7;
    let mut power = 2;
    loop {
        for i in -n.pow(power)..=n.pow(power) {
            let ray = Ray::new(*p, fibonacci_spiral(i, n.pow(power)));

            let mut nearest: Option<(f64, bool, &Entity)> = None;
            for (_name, ent) in ent_map {
                let aabb = ent.shape().aabb();
                if aabb.contains(p) || aabb.hit(&ray) {
                    if let Some((dist, inside)) = ent.shape().dist_inside(&ray) {
                        if nearest.is_some() || (dist < nearest.unwrap().0) {
                            nearest = Some((dist, inside, ent));
                        }
                    }
                }
            }

            if let Some((dist, inside, ent)) = nearest {
                if dist <= aabb.dist(&ray).unwrap() {
                    if inside {
                        return ent.in_mat();
                    }

                    return ent.out_mat();
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

    panic!(
        "Unable to observe a material from given point after {} samples.",
        (2 * n.pow(power)) + 1
    );
}
