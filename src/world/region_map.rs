//! Region-map alias.

use super::MolMap;
use crate::{
    dom::{ProtoRegion, Region, State},
    geom::shape::Aabb,
    rt::{Ray, Trace},
};
use contracts::pre;
use log::info;
use nalgebra::Point3;
use std::{collections::HashMap, path::Path};

/// Region-map alias.
pub type RegionMap = HashMap<String, Region>;

/// Construct a region-map from a hashmap of proto-regions.
#[pre(!proto_region_map.is_empty())]
#[post(!ret.is_empty())]
pub fn new_region_map(
    mesh_dir: &Path,
    proto_region_map: &HashMap<String, ProtoRegion>,
    mol_map: &MolMap,
) -> RegionMap {
    info!("Constructing the region map...");

    let mut region_map = RegionMap::with_capacity(proto_region_map.len());
    for (id, proto_react) in proto_region_map.iter() {
        info!("\tLoading region: {}", id);
        region_map.insert(
            id.to_string(),
            Region::build(mesh_dir, mol_map, &proto_react),
        );
    }

    info!("Loaded {} total regions.\n", region_map.len());

    region_map
}

/// Determine the initial concentrations and source terms for a given position.
#[pre(dom.contains(&p))]
#[pre(!mol_map.is_empty())]
#[pre(!region_map.is_empty())]
pub fn state_at_pos_from_map(
    p: Point3<f64>,
    dom: &Aabb,
    mol_map: &MolMap,
    region_map: &RegionMap,
) -> State {
    let n: i32 = 7;
    let mut power = 3;
    for i in -n.pow(power)..=n.pow(power) {
        let ray = Ray::new_fibonacci_spiral(p, i, n.pow(power));

        let mut nearest: Option<(f64, bool, &Region)> = None;
        for (_id, region) in region_map.iter() {
            if let Some((dist, inside)) = region.mesh().dist_inside(&ray) {
                if nearest.is_none() || dist < nearest.unwrap().0 {
                    nearest = Some((dist, inside, region));
                }
            }
        }

        if let Some((dist, inside, region)) = nearest {
            if inside
                && dist
                    <= dom
                        .dist(&ray)
                        .expect("Failed to determine internal dom distance.")
            {
                return region.state().clone();
            }
        }
    }

    State::new_empty(mol_map.len())
}
