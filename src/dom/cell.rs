//! Cell structure.

use super::Aabb;
use crate::{
    data::Record,
    geom::Shape,
    phys::Material,
    rt::Ray,
    world::{mat_at_pos_from_list, mat_at_pos_from_map, EntMap, Entity},
};
use contracts::pre;
use nalgebra::{Point3, Unit, Vector3};

/// Single domain cell.
#[derive(Debug)]
pub struct Cell<'a> {
    /// Record.
    rec: Record,
    /// Boundary.
    aabb: Aabb,
    /// List of intersecting entity shapes.
    ent_list: Vec<(&'a Entity<'a>, Vec<&'a Shape>)>,
    /// Central material.
    mat: &'a Material,
}

impl<'a> Cell<'a> {
    /// Construct a new instance.
    pub fn new(aabb: Aabb, ent_map: &'a EntMap, dom_aabb: &Aabb) -> Self {
        let mut ent_list = Vec::new();
        for (_name, ent) in ent_map {
            if aabb.intersect(ent.surf()) {
                let mut list = Vec::new();
                for c in ent.surf().components() {
                    if aabb.intersect(c) {
                        list.push(c);
                    }
                }

                if !list.is_empty() {
                    ent_list.push((ent, list));
                }
            }
        }

        let mat = if ent_list.is_empty() {
            mat_at_pos_from_map(&aabb.centre(), dom_aabb, ent_map)
        } else {
            mat_at_pos_from_list(&aabb.centre(), &aabb, &ent_list)
        };

        Self {
            rec: Record::new(),
            aabb,
            ent_list,
            mat,
        }
    }

    /// Reference the data record.
    pub fn rec(&self) -> &Record {
        &self.rec
    }

    /// Reference the boundary.
    pub fn aabb(&self) -> &Aabb {
        &self.aabb
    }

    /// Reference the intersecting entity list.
    pub fn ent_list(&self) -> &Vec<(&'a Entity<'a>, Vec<&'a Shape>)> {
        &self.ent_list
    }

    /// Determine the closest distance to an entity contained within the cell.
    pub fn ent_dist(&self, ray: &Ray) -> Option<f64> {
        let mut closest: Option<f64> = None;

        for (_, shapes) in self.ent_list.iter() {
            for s in shapes {
                if let Some(dist) = s.dist(ray) {
                    if closest.is_none() || (dist < closest.unwrap()) {
                        closest = Some(dist);
                    }
                }
            }
        }

        closest
    }

    /// Determine the distance to an entity, and the corresponding collision normal, contained within the cell.
    pub fn ent_dist_norm(&self, ray: &Ray) -> Option<(&'a Entity<'a>, f64, Unit<Vector3<f64>>)> {
        let mut closest: Option<(&'a Entity<'a>, f64, Unit<Vector3<f64>>)> = None;

        for (ent, shapes) in self.ent_list.iter() {
            for s in shapes {
                if let Some((dist, norm)) = s.dist_norm(ray) {
                    if closest.is_none() || (dist < closest.unwrap().1) {
                        closest = Some((ent, dist, norm));
                    }
                }
            }
        }

        closest
    }

    /// Reference the central material.
    pub fn mat(&self) -> &Material {
        &self.mat
    }

    /// Reference the material at the given position.
    #[pre(self.aabb.contains(p))]
    pub fn mat_at_pos(&self, p: &Point3<f64>) -> &Material {
        if self.ent_list.is_empty() {
            return self.mat;
        }

        mat_at_pos_from_list(&p, &self.aabb, &self.ent_list)
    }

    /// Add a record to this cell's record.
    pub fn add_record(&mut self, rec: &Record) {
        self.rec += rec;
    }
}
