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
use nalgebra::Point3;
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
    concs: Array1<f64>,
    /// Molecule sources.
    sources: Array1<f64>,
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

        Self {
            aabb,
            inter_tris,
            mat,
            rec: Record::new(),
            concs,
            sources,
        }
    }

    /// Reference the Boundary.
    pub fn aabb(&self) -> &Aabb {
        &self.aabb
    }

    /// Reference the Central material.
    pub fn mat(&self) -> &Material {
        &self.mat
    }

    /// Reference the Record.
    pub fn rec(&self) -> &Record {
        &self.rec
    }

    /// Reference the Molecule concentrations.
    pub fn concs(&self) -> &Array1<f64> {
        &self.concs
    }

    /// Reference the Molecule sources.
    pub fn sources(&self) -> &Array1<f64> {
        &self.sources
    }

    /// Check if the cell contains intersecting triangles.
    pub fn is_empty(&self) -> bool {
        self.inter_tris.is_empty()
    }

    /// Determine the material at the given position within the cell.
    #[pre(dom.contains(&p))]
    #[pre(self.aabb.contains(&p))]
    #[pre(!inter_map.is_empty())]
    pub fn mat_at_pos(&self, p: &Point3<f64>, dom: &Aabb, inter_map: &'a InterMap) -> &'a Material {
        if self.is_empty() {
            return self.mat;
        }

        mat_at_pos_from_sublist(p.clone(), dom, inter_map, &self.aabb, &self.inter_tris)
    }
}
