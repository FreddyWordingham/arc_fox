//! Cell structure.

use super::SIGMA;
use crate::{
    data::Record,
    geom::{Aabb, Collision, Triangle},
    world::{mat_at_pos_from_list, mat_at_pos_from_sublist, Entity, Material},
};
use contracts::pre;

/// Single domain cell.
#[derive(Debug)]
pub struct Cell<'a> {
    /// Boundary.
    aabb: Aabb,
    /// Record.
    rec: Record,
    /// Intersecting entity triangles.
    ent_tris: Vec<(&'a Entity<'a>, Vec<&'a Triangle>)>,
    /// Central material.
    mat: &'a Material,
}

impl<'a> Cell<'a> {
    /// Construct a new instance.
    #[pre(!ents.is_empty())]
    pub fn new(dom: &Aabb, ents: &'a Vec<Entity>, aabb: Aabb) -> Self {
        let det_box = aabb.loosen(SIGMA);
        let mut ent_tris = Vec::new();
        for ent in ents {
            if ent.mesh().overlap(&det_box) {
                let mut list = Vec::new();
                for tri in ent.mesh().tris() {
                    if tri.overlap(&det_box) {
                        list.push(tri);
                    }
                }

                if !list.is_empty() {
                    ent_tris.push((ent, list));
                }
            }
        }

        let mat = if ent_tris.is_empty() {
            mat_at_pos_from_list(aabb.centre(), &dom, ents)
        } else {
            mat_at_pos_from_sublist(aabb.centre(), &dom, ents, &det_box, &ent_tris)
        };

        Self {
            aabb,
            rec: Record::new(),
            ent_tris,
            mat,
        }
    }

    /// Check if the cell contains intersecting triangles.
    pub fn is_empty(&self) -> bool {
        self.ent_tris.is_empty()
    }

    /// Reference the central material.
    pub fn mat(&self) -> &Material {
        &self.mat
    }

    /// Add a record to this cell's record.
    pub fn add_record(&mut self, rec: &Record) {
        self.rec += rec;
    }
}
