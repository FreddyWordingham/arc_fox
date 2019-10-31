//! Domain cell structure.

use super::{EntMap, Entity};
use crate::geom::{Aabb, Collidable, Shape};
use contracts::pre;

/// Domain cell structure.
/// Contains local spatial information.
pub struct Cell<'a> {
    /// Boundary.
    boundary: Aabb,
    /// Intersecting entity shapes.
    ents: Option<Vec<(&'a Entity<'a>, Vec<&'a Box<dyn Shape>>)>>,
}

impl<'a> Cell<'a> {
    /// Construct a new instance.
    pub fn new(boundary: Aabb, ent_map: &'a EntMap<'a>) -> Self {
        let mut ents_list = Vec::new();
        for (_name, ent) in ent_map.iter() {
            if boundary.collides(ent.boundary()) {
                let mut surfs = Vec::new();
                for surf in ent.surfs() {
                    if surf.collides(&boundary) {
                        surfs.push(surf);
                    }
                }

                if !surfs.is_empty() {
                    ents_list.push((ent, surfs));
                }
            }
        }

        let ents = if ents_list.is_empty() {
            None
        } else {
            Some(ents_list)
        };

        Self { boundary, ents }
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
}
