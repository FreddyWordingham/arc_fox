//! World entity structure.

use crate::{geom::Shape, phys::Material};

/// World entity structure.
/// Binds a material to a shape.
pub struct Entity<'a> {
    /// Surface shape.
    shape: Shape,
    /// Inside material.
    in_mat: &'a Material,
    /// Outside material.
    out_mat: &'a Material,
}

impl<'a> Entity<'a> {
    /// Construct a new instance.
    pub fn new(shape: Shape, in_mat: &'a Material, out_mat: &'a Material) -> Self {
        Self {
            shape,
            in_mat,
            out_mat,
        }
    }
}
