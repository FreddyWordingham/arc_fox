//! Universe structure.

#![allow(unused_variables)]

use super::{
    new_inter_map, new_mat_map, new_mol_map, new_react_map, new_region_map, InterMap, MatMap,
    MolMap, ReactMap,
};
use crate::{
    chem::ProtoReaction,
    dom::{Grid, ProtoGrid, ProtoRegion},
    json,
    mat::ProtoInterface,
};
use contracts::{post, pre};
use log::info;
use self_ref::self_referencing;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path, sync::Arc};

/// Universe structure implementation.
#[derive(Debug)]
pub struct Universe<'a> {
    /// Molecule-map.
    mol_map: MolMap,
    /// Reaction-map.
    react_map: ReactMap,
    /// Material-map.
    mat_map: MatMap,
    /// Interface-map.
    inter_map: InterMap<'a>,
    /// Grid of cells.
    grid: Grid<'a>,
}

impl<'a> Universe<'a> {
    /// Build a new instance.
    pub fn build(input_dir: &Path, proto_uni: &ProtoUniverse, num_threads: usize) -> Self {
        info!("Building universe...\n");

        let uni = Arc::try_unwrap(self_referencing!(Universe, {
            mol_map = new_mol_map(&input_dir.join("mols"), proto_uni.mol_list());
            react_map = new_react_map(&proto_uni.react_map, &mol_map);
            mat_map = new_mat_map(&input_dir.join("mats"), proto_uni.mat_list());
            inter_map = new_inter_map(&input_dir.join("meshes"), &proto_uni.inter_map, &mat_map);
            grid = Grid::build(
                &proto_uni.grid,
                &inter_map,
                &mol_map,
                &new_region_map(&input_dir.join("meshes"), &proto_uni.region_map, &mol_map),
                num_threads,
            );
        }))
        .expect("Could not create universe instance.");

        info!("Universe constructed.");

        uni
    }

    /// Reference the molecule-map.
    pub fn mol_map(&self) -> &MolMap {
        &self.mol_map
    }

    /// Reference the reaction-map.
    pub fn react_map(&self) -> &ReactMap {
        &self.react_map
    }

    /// Reference the material-map.
    pub fn mat_map(&self) -> &MatMap {
        &self.mat_map
    }

    /// Reference the interface-map.
    pub fn inter_map(&self) -> &InterMap {
        &self.inter_map
    }

    /// Reference the grid of cells.
    pub fn grid(&self) -> &Grid<'a> {
        &self.grid
    }
}

/// Proto-Universe structure implementation.
/// Used to build universes.
#[derive(Debug, Deserialize, Serialize)]
pub struct ProtoUniverse {
    /// Grid.
    grid: ProtoGrid,
    /// Reactions.
    react_map: HashMap<String, ProtoReaction>,
    /// Interfaces.
    inter_map: HashMap<String, ProtoInterface>,
    /// Regions to initialise.
    region_map: HashMap<String, ProtoRegion>,
}

impl ProtoUniverse {
    /// Construct a new instance.
    #[pre(!inter_map.is_empty())]
    pub fn new(
        grid: ProtoGrid,
        react_map: HashMap<String, ProtoReaction>,
        inter_map: HashMap<String, ProtoInterface>,
        region_map: HashMap<String, ProtoRegion>,
    ) -> Self {
        Self {
            grid,
            react_map,
            inter_map,
            region_map,
        }
    }

    /// Reference the proto-grid.
    pub fn grid(&self) -> &ProtoGrid {
        &self.grid
    }

    /// Construct a list of molecule names.
    pub fn mol_list(&self) -> Vec<String> {
        let mut mol_list = Vec::new();

        for (_id, react) in self.react_map.iter() {
            for reactant in react.reactants().iter() {
                mol_list.push(reactant.1.clone());
            }
            for product in react.products().iter() {
                mol_list.push(product.1.clone());
            }
            for depend in react.rate().dependants().iter() {
                mol_list.push(depend.to_string());
            }
        }

        for (_id, region) in self.region_map.iter() {
            for (mol_id, _conc) in region.state().concs().iter() {
                mol_list.push(mol_id.to_string());
            }
            for (mol_id, _source) in region.state().sources().iter() {
                mol_list.push(mol_id.to_string());
            }
        }

        mol_list.sort();
        mol_list.dedup();

        mol_list
    }

    /// Construct a list of material names.
    #[post(!ret.is_empty())]
    pub fn mat_list(&self) -> Vec<String> {
        let mut mat_list = Vec::new();

        for (_id, inter) in self.inter_map.iter() {
            mat_list.push(inter.in_mat().to_string());
            mat_list.push(inter.out_mat().to_string());
        }

        mat_list.sort();
        mat_list.dedup();

        mat_list
    }
}

json!(ProtoUniverse);
