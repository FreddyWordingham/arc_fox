//! Cell structure.

use super::SIGMA;
use crate::{
    data::Record,
    geom::{
        shape::{Aabb, Triangle},
        Collide,
    },
    mat::{Interface, Material},
    world::{
        concs_sources_from_map, mat_at_pos_from_map, mat_at_pos_from_sublist, InterMap, MolMap,
        RegionMap,
    },
};
use contracts::pre;
use ndarray::Array1;

/// Cell structure implementation.
#[derive(Debug)]
pub struct Cell<'a> {
    /// Boundary.
    aabb: Aabb,
    /// Intersecting interface triangles.
    inter_tris: Vec<(&'a Interface<'a>, Vec<&'a Triangle>)>,
    /// Central material.
    mat: &'a Material,
    /// Record.
    rec: Record,
    /// Molecule concentrations.
    mol_concs: Array1<f64>,
    /// Molecule sources.
    mol_sources: Array1<f64>,
}

impl<'a> Cell<'a> {
    /// Construct a new instance.
    #[pre(!inter_map.is_empty())]
    #[pre(!mol_map.is_empty())]
    #[pre(!region_map.is_empty())]
    pub fn new(
        dom: &Aabb,
        inter_map: &'a InterMap,
        mol_map: &'a MolMap,
        region_map: &RegionMap,
        aabb: Aabb,
    ) -> Self {
        let mut inter_tris = Vec::new();
        let det_box = aabb.loosen(SIGMA);
        for (_id, inter) in inter_map.iter() {
            if inter.mesh().overlap(&det_box) {
                let mut list = Vec::new();
                for tri in inter.mesh().tris().iter() {
                    if tri.overlap(&det_box) {
                        list.push(tri);
                    }
                }

                if !list.is_empty() {
                    inter_tris.push((inter, list));
                }
            }
        }

        let mat = if inter_tris.is_empty() {
            mat_at_pos_from_map(aabb.centre(), &dom, inter_map)
        } else {
            mat_at_pos_from_sublist(aabb.centre(), &dom, inter_map, &det_box, &inter_tris)
        };

        let (concs, sources) = concs_sources_from_map(aabb.centre(), &dom, mol_map, region_map);

        let mut mol_concs = Array1::zeros(mol_map.len());
        let mut mol_sources = Array1::zeros(mol_map.len());
        for (id, region) in region_map.iter() {}

        Self {
            aabb,
            inter_tris,
            mat,
            rec: Record::new(),
            mol_concs,
            mol_sources,
        }
    }
}
