//! Entity map alias.

use super::{Entity, MatMap};
use crate::{
    dom::Aabb,
    geom::Shape,
    phys::Material,
    proto::Entity as ProtoEntity,
    rt::{fibonacci_spiral, Ray, Traceable},
};
use contracts::pre;
use log::{info, warn};
use nalgebra::Point3;
use std::collections::HashMap;

/// Entity list alias type.
pub type EntMap<'a> = HashMap<String, Entity<'a>>;

#[pre(!list.is_empty())]
#[post(!ret.is_empty())]
pub fn load_ent_map<'a>(list: Vec<ProtoEntity>, mat_map: &'a MatMap) -> EntMap<'a> {
    let mut ent_map = EntMap::with_capacity(list.len());

    info!("Constructing entities...");
    for proto_ent in list {
        ent_map.insert(
            format!("entity_{}", ent_map.len()),
            proto_ent.manifest(mat_map),
        );
    }
    info!("{} entities constructed.", ent_map.len());

    ent_map
}

#[pre(aabb.contains(p))]
#[pre(!ent_map.is_empty())]
pub fn mat_at_point_from_map<'a>(
    p: &Point3<f64>,
    aabb: &Aabb,
    ent_map: &'a EntMap,
) -> &'a Material {
    let n: i32 = 7;
    let mut power = 2;
    loop {
        for i in -n.pow(power)..=n.pow(power) {
            let ray = Ray::new(*p, fibonacci_spiral(i, n.pow(power)));

            let mut nearest: Option<(f64, bool, &Entity)> = None;
            for (_name, ent) in ent_map {
                let ent_aabb = ent.surf().aabb();
                if ent_aabb.contains(p) || ent_aabb.hit(&ray) {
                    if let Some((dist, inside)) = ent.surf().dist_inside(&ray) {
                        if nearest.is_none() || (dist < nearest.unwrap().0) {
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

#[pre(aabb.contains(p))]
#[pre(!ent_map.is_empty())]
pub fn mat_at_point_from_list<'a>(
    p: &Point3<f64>,
    aabb: &Aabb,
    ent_map: &Vec<(&'a Entity, Vec<&Shape>)>,
) -> &'a Material {
    let n: i32 = 7;
    let mut power = 2;
    loop {
        for i in -n.pow(power)..=n.pow(power) {
            let ray = Ray::new(*p, fibonacci_spiral(i, n.pow(power)));

            let mut nearest: Option<(f64, bool, &Entity)> = None;
            for (ent, shapes) in ent_map {
                for s in shapes {
                    if let Some((dist, inside)) = s.dist_inside(&ray) {
                        if nearest.is_none() || (dist < nearest.unwrap().0) {
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
