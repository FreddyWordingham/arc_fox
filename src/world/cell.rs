//! Domain cell structure.

use super::{EntMap, Entity};
use crate::geom::{Aabb, Collidable, Shape};
use log::warn;

/// Domain cell structure.
/// Contains local spatial information.
pub struct Cell<'a> {
    /// Boundary.
    boundary: Aabb,
    /// Entity shapes.
    ents: Option<Vec<(&'a Entity<'a>, Vec<&'a Box<dyn Shape>>)>>,
}

impl<'a> Cell<'a> {
    /// Construct a new instance.
    pub fn new(boundary: Aabb, ent_map: &'a EntMap<'a>) -> Self {
        warn!("Ents could be culled here.");

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

    /// Determine if the cell contains intersecting entity surfaces.
    pub fn is_empty(&self) -> bool {
        self.ents.is_none()
    }
}
