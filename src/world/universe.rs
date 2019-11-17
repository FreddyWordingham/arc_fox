//! Universe structure.

use super::{new_mat_map, new_mol_map, MatMap};
use crate::{base::Resolution, chem::ProtoReaction, json, mat::ProtoInterface};
use contracts::{post, pre};
use log::info;
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

/// Universe structure implementation.
#[derive(Debug)]
pub struct Universe {
    // Reaction-map.
    // react_map: ReactMap,
    // Material-map.
    mat_map: MatMap,
}

impl Universe {
    /// Construct a new instance.
    pub fn build(input_dir: &Path, proto_uni: &ProtoUniverse) -> Self {
        info!("Building universe...");

        let mol_map = new_mol_map(&input_dir.join("mols"), proto_uni.mol_list());
        // let react_map = new_react_map(react_dir, proto_uni.react_list());
        let mat_map = new_mat_map(&input_dir.join("mats"), proto_uni.mat_list());

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
    /// Reactions.
    reacts: Vec<ProtoReaction>,
    /// Interfaces.
    inters: Vec<ProtoInterface>,
}

impl ProtoUniverse {
    /// Construct a new instance.
    #[pre(!inters.is_empty())]
    #[pre(half_extents.iter().all(|x| *x > 0.0))]
    pub fn new(
        res: Resolution,
        half_extents: Vector3<f64>,
        reacts: Vec<ProtoReaction>,
        inters: Vec<ProtoInterface>,
    ) -> Self {
        Self {
            res,
            half_extents,
            reacts,
            inters,
        }
    }

    /// Reference the list of reactions.
    #[post(!ret.is_empty())]
    fn reacts(&self) -> &Vec<ProtoReaction> {
        &self.reacts
    }

    /// Construct a list of molecule names.
    #[post(!ret.is_empty())]
    fn mol_list(&self) -> Vec<String> {
        let mut mol_list = Vec::new();

        for react in self.reacts.iter() {
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

        for inter in self.inters.iter() {
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

        for inter in self.inters.iter() {
            mat_list.push(inter.in_mat().to_string());
            mat_list.push(inter.out_mat().to_string());
        }

        mat_list.sort();
        mat_list.dedup();

        mat_list
    }
}

json!(ProtoUniverse);
