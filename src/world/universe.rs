//! Universe structure.

use super::{new_mat_map, new_mol_map, new_react_map, MatMap, MolMap, ReactMap};
use crate::{base::Resolution, chem::ProtoReaction, json, mat::ProtoInterface};
use contracts::{post, pre};
use log::info;
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Universe structure implementation.
#[derive(Debug)]
pub struct Universe {
    /// Molecule-map.
    mol_map: MolMap,
    // Reaction-map.
    react_map: ReactMap,
    /// Material-map.
    mat_map: MatMap,
}

impl Universe {
    /// Construct a new instance.
    pub fn build(input_dir: &Path, proto_uni: &ProtoUniverse) -> Self {
        info!("Building universe...");

        let mol_map = new_mol_map(&input_dir.join("mols"), proto_uni.mol_list());
        let react_map = new_react_map(&input_dir.join("reacts"), proto_uni.react_map(), &mol_map);
        let mat_map = new_mat_map(&input_dir.join("mats"), proto_uni.mat_list());

        Self {
            mol_map,
            react_map,
            mat_map,
        }
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
    /// Reactions.
    react_map: HashMap<String, ProtoReaction>,
    /// Interfaces.
    inters: HashMap<String, ProtoInterface>,
}

impl ProtoUniverse {
    /// Construct a new instance.
    #[pre(!inters.is_empty())]
    #[pre(half_extents.iter().all(|x| *x > 0.0))]
    pub fn new(
        res: Resolution,
        half_extents: Vector3<f64>,
        react_map: HashMap<String, ProtoReaction>,
        inters: HashMap<String, ProtoInterface>,
    ) -> Self {
        Self {
            res,
            half_extents,
            react_map,
            inters,
        }
    }

    /// Reference the reactions map.
    #[post(!ret.is_empty())]
    fn react_map(&self) -> &HashMap<String, ProtoReaction> {
        &self.react_map
    }

    /// Construct a list of molecule names.
    #[post(!ret.is_empty())]
    fn mol_list(&self) -> Vec<String> {
        let mut mol_list = Vec::new();

        for (_id, react) in self.react_map.iter() {
            for reactant in react.reactants().iter() {
                mol_list.push(reactant.1.clone());
            }
            for product in react.reactants().iter() {
                mol_list.push(product.1.clone());
            }
            for depend in react.rate().dependants().iter() {
                mol_list.push(depend.to_string());
            }
        }

        for (_id, _inter) in self.inters.iter() {
            // Todo: Get any concentrations set within the material.
        }

        mol_list.sort();
        mol_list.dedup();

        mol_list
    }

    /// Construct a list of material names.
    #[post(!ret.is_empty())]
    fn mat_list(&self) -> Vec<String> {
        let mut mat_list = Vec::new();

        for (_id, inter) in self.inters.iter() {
            mat_list.push(inter.in_mat().to_string());
            mat_list.push(inter.out_mat().to_string());
        }

        mat_list.sort();
        mat_list.dedup();

        mat_list
    }
}

json!(ProtoUniverse);
