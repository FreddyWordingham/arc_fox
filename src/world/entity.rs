//! World entity structure.

use crate::{geom::Shape, phys::Material};

/// World entity structure.
/// Binds a material to a shape.
#[derive(Debug)]
pub struct Entity<'a> {
    /// Surface shape.
    surf: Shape,
    /// Inside material.
    in_mat: &'a Material,
    /// Outside material.
    out_mat: &'a Material,
}

impl<'a> Entity<'a> {
    /// Construct a new instance.
    pub fn new(surf: Shape, in_mat: &'a Material, out_mat: &'a Material) -> Self {
        Self {
            surf,
            in_mat,
            out_mat,
        }
    }

    /// Reference the surface shape.
    pub fn surf(&self) -> &Shape {
        &self.surf
    }

    /// Reference the inside material.
    pub fn in_mat(&self) -> &Material {
        &self.in_mat
    }

    /// Reference the outside material.
    pub fn out_mat(&self) -> &Material {
        &self.out_mat
    }
}
