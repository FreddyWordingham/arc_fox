//! Domain cell structure.

use super::Entity;
use crate::geom::{Aabb, Collidable, Shape};
use log::warn;

/// Domain cell structure.
/// Contains local spatial information.
pub struct Cell<'a> {
    /// Boundary.
    boundary: Aabb,
    /// Entity shapes.
    ents: Vec<(&'a Entity<'a>, Vec<&'a Box<dyn Shape>>)>,
}

impl<'a> Cell<'a> {
    /// Construct a new instance.
    pub fn new(boundary: Aabb, ents: &'a Vec<Entity<'a>>) -> Self {
        warn!("Ents could be culled here.");

        let mut ents_list = Vec::new();
        for ent in ents {
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

        Self {
            boundary,
            ents: ents_list,
        }
    }
}
