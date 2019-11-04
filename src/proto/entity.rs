//! Proto-Entity structure.

use crate::{
    geom::Shape,
    world::{Entity as WorldEntity, MatMap},
};

/// Proto-Entity structure.
/// Used to manifest world entities.
pub struct Entity {
    /// Entity surface shape.
    pub surf: Shape,
    /// Inside material keyname.
    pub in_mat: &'static str,
    /// Outside material keyname.
    pub out_mat: &'static str,
}

impl Entity {
    /// Construct a new instance.
    pub fn new(surf: Shape, in_mat: &'static str, out_mat: &'static str) -> Self {
        Self {
            surf,
            in_mat,
            out_mat,
        }
    }

    /// Manifest into a world entity.
    pub fn manifest(self, mat_map: &MatMap) -> WorldEntity {
        WorldEntity::new(self.surf, &mat_map[self.in_mat], &mat_map[self.out_mat])
    }
}
