//! Cell structure.

use crate::{
    data::Record,
    geom::{Aabb, Collision, Triangle},
    world::Entity,
};

/// Single domain cell.
pub struct Cell<'a> {
    /// Boundary.
    aabb: Aabb,
    /// Record.
    rec: Record,
    /// Intersecting entity triangles.
    ent_tris: Vec<(&'a Entity<'a>, Vec<&'a Triangle>)>,
}

impl<'a> Cell<'a> {
    /// Construct a new instance.
    pub fn new(aabb: Aabb, ents: &'a Vec<Entity>) -> Self {
        let mut ent_tris = Vec::new();
        for ent in ents {
            if ent.mesh().overlap(&aabb) {
                let mut list = Vec::new();
                for tri in ent.mesh().tris() {
                    if tri.overlap(&aabb) {
                        list.push(tri);
                    }
                }

                if !list.is_empty() {
                    ent_tris.push((ent, list));
                }
            }
        }

        Self {
            aabb,
            rec: Record::new(),
            ent_tris,
        }
    }
}
