//! Universe structure.

use super::{new_mat_map, MatMap};
use crate::{base::Resolution, json, mat::ProtoInterface};
use contracts::{post, pre};
use log::info;
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

/// Universe structure implementation.
#[derive(Debug)]
pub struct Universe {
    // Material-map.
    mat_map: MatMap,
}

impl Universe {
    /// Construct a new instance.
    pub fn build(mat_dir: &Path, proto_uni: &ProtoUniverse) -> Self {
        info!("Building universe...");

        let mat_map = new_mat_map(mat_dir, proto_uni.mat_list());

        Self { mat_map }
    }
}

/// Proto-Universe structure implementation.
/// Used to build universes.
#[derive(Debug, Deserialize, Serialize)]
pub struct ProtoUniverse {
    /// Grid resolution.
    res: Resolution,
    /// Half-extents.
    half_extents: Vector3<f64>,
    /// Interfaces.
    inters: Vec<ProtoInterface>,
}

impl ProtoUniverse {
    /// Construct a new instance.
    #[pre(!inters.is_empty())]
    #[pre(half_extents.iter().all(|x| *x > 0.0))]
    pub fn new(res: Resolution, half_extents: Vector3<f64>, inters: Vec<ProtoInterface>) -> Self {
        Self {
            res,
            half_extents,
            inters,
        }
    }

    /// Construct a list of material names.
    #[post(!ret.is_empty())]
    fn mat_list(&self) -> Vec<&str> {
        let mut mat_list = Vec::new();

        for inter in self.inters.iter() {
            mat_list.push(inter.in_mat());
            mat_list.push(inter.out_mat());
        }

        mat_list.sort();
        mat_list.dedup();

        mat_list
    }
}

json!(ProtoUniverse);
