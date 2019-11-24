//! Cell structure.

use super::{State, SIGMA};
use crate::{
    geom::{
        shape::{Aabb, Triangle},
        Collide,
    },
    mat::{Interface, Material},
    rt::{Ray, Trace},
    world::{
        mat_at_pos_from_map, mat_at_pos_from_sublist, state_at_pos_from_map, InterMap, MolMap,
        RegionMap,
    },
};
use contracts::pre;
use nalgebra::{Point3, Unit, Vector3};

/// Cell structure implementation.
#[derive(Debug)]
pub struct Cell<'a> {
    /// Boundary.
    aabb: Aabb,
    /// Intersecting interface triangles.
    inter_tris: Vec<(&'a Interface<'a>, Vec<&'a Triangle>)>,
    /// Central material.
    mat: &'a Material,
    /// Current physical state.
    state: State,
}

impl<'a> Cell<'a> {
    /// Construct a new instance.
    #[pre(!inter_map.is_empty())]
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

        let state = state_at_pos_from_map(aabb.centre(), &dom, mol_map, region_map);

        Self {
            aabb,
            inter_tris,
            mat,
            state,
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

    /// Reference the state.
    pub fn state(&self) -> &State {
        &self.state
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

    /// Determine the distance to an interface contained within the cell.
    pub fn inter_dist(&self, ray: &Ray) -> Option<f64> {
        let mut closest = None;

        for (_inter, tris) in self.inter_tris.iter() {
            for tri in tris {
                if let Some(dist) = tri.dist(ray) {
                    if closest.is_none() || (dist < closest.unwrap()) {
                        closest = Some(dist);
                    }
                }
            }
        }

        closest
    }

    /// Determine the distance to an interface contained within the cell, if hitting on the inside of the interface, and the normal at the intersection point.
    pub fn inter_dist_inside_norm_inter(
        &self,
        ray: &Ray,
    ) -> Option<(f64, bool, Unit<Vector3<f64>>, &Interface)> {
        let mut closest: Option<(f64, bool, Unit<Vector3<f64>>, &Interface)> = None;

        for (inter, tris) in self.inter_tris.iter() {
            for tri in tris {
                if let Some((dist, inside, norm)) = tri.dist_inside_norm(ray) {
                    if closest.is_none() || (dist < closest.unwrap().0) {
                        closest = Some((dist, inside, norm, inter));
                    }
                }
            }
        }

        closest
    }
}
