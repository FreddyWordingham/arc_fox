//! Domain cell structure.

use super::{mat_at_point, EntMap, Entity};
use crate::{
    geom::{Aabb, Collidable, Shape},
    phys::Material,
};
use contracts::pre;

/// Domain cell structure.
/// Contains local spatial information.
pub struct Cell<'a> {
    /// Boundary.
    boundary: Aabb,
    /// Intersecting entity shapes.
    ents: Option<Vec<(&'a Entity<'a>, Vec<&'a Box<dyn Shape>>)>>,
    /// Default material.
    mat: &'a Material,
}

impl<'a> Cell<'a> {
    /// Construct a new instance.
    pub fn new(dom_bound: &Aabb, boundary: Aabb, ent_map: &'a EntMap<'a>) -> Self {
        let ents = Self::init_ents(&boundary, ent_map);
        let centre = boundary.centre();

        Self {
            boundary,
            ents,
            mat: mat_at_point(&centre, dom_bound, ent_map),
        }
    }

    /// Initialise the list of colliding entity shapes.
    fn init_ents(
        boundary: &Aabb,
        ent_map: &'a EntMap<'a>,
    ) -> Option<Vec<(&'a Entity<'a>, Vec<&'a Box<dyn Shape>>)>> {
        let mut ents = Vec::new();
        for (_name, ent) in ent_map.iter() {
            if boundary.collides(ent.boundary()) {
                let mut surfs = Vec::new();
                for surf in ent.surfs() {
                    if surf.collides(&boundary) {
                        surfs.push(surf);
                    }
                }

                if !surfs.is_empty() {
                    ents.push((ent, surfs));
                }
            }
        }

        if ents.is_empty() {
            return None;
        }

        Some(ents)
    }

    /// Reference the intersecting entity shapes.
    #[pre(self.ents.is_some())]
    pub fn ents(&self) -> &Vec<(&'a Entity<'a>, Vec<&'a Box<dyn Shape>>)> {
        self.ents.as_ref().unwrap()
    }

    /// Determine if the cell contains intersecting entity surfaces.
    pub fn is_empty(&self) -> bool {
        self.ents.is_none()
    }

    /// Reference the central material.
    pub fn mat(&self) -> &Material {
        &self.mat
    }
}
