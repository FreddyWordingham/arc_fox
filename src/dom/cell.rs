//! Cell structure.

use super::SIGMA;
use crate::{
    data::Record,
    geom::{
        shape::{Aabb, Triangle},
        Collide,
    },
    mat::{Interface, Material},
    world::{mat_at_pos_from_map, mat_at_pos_from_sublist, InterMap},
};
use contracts::pre;

/// Cell structure implementation.
#[derive(Debug)]
pub struct Cell<'a> {
    /// Boundary.
    aabb: Aabb,
    /// Record.
    rec: Record,
    /// Intersecting interface triangles.
    inter_tris: Vec<(&'a Interface<'a>, Vec<&'a Triangle>)>,
    /// Central material.
    mat: &'a Material,
}

impl<'a> Cell<'a> {
    /// Construct a new instance.
    #[pre(!inter_map.is_empty())]
    pub fn new(dom: &Aabb, inter_map: &'a InterMap, aabb: Aabb) -> Self {
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

        Self {
            aabb,
            rec: Record::new(),
            inter_tris,
            mat,
        }
    }
}
