//! Cell structure.

use super::SIGMA;
use crate::{
    data::Record,
    geom::{Aabb, Collision, Triangle},
    rt::{Ray, Traceable},
    world::{mat_at_pos_from_list, mat_at_pos_from_sublist, Entity, Material},
};
use contracts::pre;
use nalgebra::{Point3, Unit, Vector3};

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

    /// Reference the boundary.
    pub fn aabb(&self) -> &Aabb {
        &self.aabb
    }

    /// Reference the record.
    pub fn rec(&self) -> &Record {
        &self.rec
    }

    /// Reference the central material.
    pub fn mat(&self) -> &Material {
        &self.mat
    }

    /// Check if the cell contains intersecting triangles.
    pub fn is_empty(&self) -> bool {
        self.ent_tris.is_empty()
    }

    /// Determine the material at the given position within the cell.
    #[pre(dom.contains(&p))]
    #[pre(self.aabb.contains(&p))]
    #[pre(!ents.is_empty())]
    pub fn mat_at_pos(&self, p: &Point3<f64>, dom: &Aabb, ents: &'a Vec<Entity>) -> &'a Material {
        mat_at_pos_from_sublist(p.clone(), dom, ents, &self.aabb, &self.ent_tris)
    }

    /// Determine the distance to an entity contained within the cell.
    pub fn ent_dist(&self, ray: &Ray) -> Option<f64> {
        let mut closest = None;

        for (_ent, tris) in self.ent_tris.iter() {
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

    /// Determine the distance to an entity, and the corresponding collision normal, contained within the cell.
    pub fn ent_dist_norm_ent(
        &self,
        ray: &Ray,
    ) -> Option<(f64, Unit<Vector3<f64>>, &'a Entity<'a>)> {
        let mut closest: Option<(f64, Unit<Vector3<f64>>, &'a Entity<'a>)> = None;

        for (ent, tris) in self.ent_tris.iter() {
            for tri in tris {
                if let Some((dist, norm)) = tri.dist_norm(ray) {
                    if closest.is_none() || (dist < closest.unwrap().0) {
                        closest = Some((dist, norm, ent));
                    }
                }
            }
        }

        closest
    }

    /// Add a record to this cell's record.
    pub fn add_record(&mut self, rec: &Record) {
        self.rec += rec;
    }
}
