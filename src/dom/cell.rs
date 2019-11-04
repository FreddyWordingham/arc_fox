//! Cell structure.

use super::Aabb;
use crate::{
    geom::Shape,
    phys::Material,
    world::{mat_at_point_from_list, mat_at_point_from_map, EntMap, Entity},
};
use contracts::pre;
use nalgebra::Point3;

/// Single domain cell.
pub struct Cell<'a> {
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
            mat_at_point_from_map(&aabb.centre(), dom_aabb, ent_map)
        } else {
            mat_at_point_from_list(&aabb.centre(), &aabb, &ent_list)
        };

        Self {
            aabb,
            ent_list,
            mat,
        }
    }

    /// Reference the boundary.
    pub fn aabb(&self) -> &Aabb {
        &self.aabb
    }

    /// Reference the intersecting entity list.
    pub fn ent_list(&self) -> &Vec<(&'a Entity<'a>, Vec<&'a Shape>)> {
        &self.ent_list
    }

    /// Reference the central material.
    pub fn mat(&self) -> &Material {
        &self.mat
    }

    /// Reference the material at the given point.
    #[pre(self.aabb.contains(p))]
    pub fn mat_at_point(&self, p: &Point3<f64>) -> &Material {
        if self.ent_list.is_empty() {
            return self.mat;
        }

        mat_at_point_from_list(&p, &self.aabb, &self.ent_list)
    }
}
