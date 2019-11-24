//! Interface-map alias.

use super::MatMap;
use crate::{
    geom::shape::{Aabb, Triangle},
    mat::{Interface, Material, ProtoInterface},
    rt::{Ray, Trace},
    util::progress::bar,
};
use contracts::pre;
use log::warn;
use nalgebra::Point3;
use std::{collections::HashMap, path::Path};

/// Interface-map alias.
pub type InterMap<'a> = HashMap<String, Interface<'a>>;

/// Construct a interface-map from a hashmap of proto-interfaces.
#[pre(mesh_dir.is_dir())]
#[pre(!proto_inter_map.is_empty())]
#[post(!ret.is_empty())]
pub fn new_inter_map<'a>(
    mesh_dir: &Path,
    proto_inter_map: &HashMap<String, ProtoInterface>,
    mat_map: &'a MatMap,
) -> InterMap<'a> {
    let pb = bar("Constructing interfaces", proto_inter_map.len() as u64);

    let mut inter_map = InterMap::with_capacity(proto_inter_map.len());
    for (id, proto_inter) in proto_inter_map.iter() {
        pb.inc(1);

        inter_map.insert(
            id.to_string(),
            Interface::build(mesh_dir, proto_inter, mat_map),
        );
    }

    pb.finish_with_message("Interfaces constructed.");

    inter_map
}

#[pre(dom.contains(&p))]
#[pre(!inter_map.is_empty())]
pub fn mat_at_pos_from_map<'a>(
    p: Point3<f64>,
    dom: &Aabb,
    inter_map: &'a InterMap,
) -> &'a Material {
    let n: i32 = 7;
    let mut power = 2;
    loop {
        for i in -n.pow(power)..=n.pow(power) {
            let ray = Ray::new_fibonacci_spiral(p, i, n.pow(power));

            let mut nearest: Option<(f64, bool, &Interface)> = None;
            for (_id, inter) in inter_map.iter() {
                if let Some((dist, inside)) = inter.mesh().dist_inside(&ray) {
                    if nearest.is_none() || dist < nearest.unwrap().0 {
                        nearest = Some((dist, inside, inter));
                    }
                }
            }

            if let Some((dist, inside, inter)) = nearest {
                if dist
                    <= dom
                        .dist(&ray)
                        .expect("Failed to determine internal dom distance.")
                {
                    return if inside {
                        inter.in_mat()
                    } else {
                        inter.out_mat()
                    };
                }
            }
        }

        if power > 5 {
            break;
        } else if power > 3 {
            warn!(
                "Increasing world-casting power to {} ({} rays)",
                power,
                (2 * n.pow(power)) + 1
            );
        }
        power += 1;
    }

    panic!(
        "Unable to observe a material from given point after {} samples.",
        (2 * n.pow(power)) + 1
    );
}

#[pre(dom.contains(&p))]
#[pre(!inter_map.is_empty())]
#[pre(cell.contains(&p))]
#[pre(!inter_tris.is_empty())]
pub fn mat_at_pos_from_sublist<'a>(
    p: Point3<f64>,
    dom: &Aabb,
    inter_map: &'a InterMap,
    cell: &Aabb,
    inter_tris: &Vec<(&'a Interface, Vec<&Triangle>)>,
) -> &'a Material {
    let n: i32 = 7;
    let mut power = 2;
    loop {
        for i in -n.pow(power)..=n.pow(power) {
            let ray = Ray::new_fibonacci_spiral(p, i, n.pow(power));

            let mut nearest: Option<(f64, bool, &Interface)> = None;
            for (inter, tris) in inter_tris.iter() {
                for tri in tris.iter() {
                    if let Some((dist, inside)) = tri.dist_inside(&ray) {
                        if nearest.is_none() || dist < nearest.unwrap().0 {
                            nearest = Some((dist, inside, inter));
                        }
                    }
                }
            }

            if let Some((dist, inside, inter)) = nearest {
                if dist
                    <= cell
                        .dist(&ray)
                        .expect("Failed to determine internal cell distance.")
                {
                    return if inside {
                        inter.in_mat()
                    } else {
                        inter.out_mat()
                    };
                }
            }
        }

        if power > 4 {
            break;
        }
        power += 1;
    }

    warn!("Falling back on world-cast.");
    mat_at_pos_from_map(p, dom, inter_map)
}
