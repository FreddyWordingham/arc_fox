//! Cell structure.

use super::SIGMA;
use crate::{
    data::Record,
    geom::{Aabb, Collision, Triangle},
    world::Entity,
};
use contracts::pre;

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
    #[pre(!ents.is_empty())]
    pub fn new(ents: &'a Vec<Entity>, aabb: Aabb) -> Self {
        let mut ent_tris = Vec::new();
        let detection_box = aabb.loosen(SIGMA);
        for ent in ents {
            if ent.mesh().overlap(&detection_box) {
                let mut list = Vec::new();
                for tri in ent.mesh().tris() {
                    if tri.overlap(&detection_box) {
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
