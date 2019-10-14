//! Material entity structure.

use crate::{geom::Triangle, phy::Material};

/// Material entity structure forming the boundary between two materials.
#[derive(Debug)]
pub struct Entity<'a> {
    /// Bounding triangles.
    tris: Vec<Triangle>,
    /// Outside material.
    out_mat: &'a Material,
    /// Inside material.
    in_mat: &'a Material,
}

impl<'a> Entity<'a> {
    /// Construct a new instance.
    pub fn new(tris: Vec<Triangle>, out_mat: &'a Material, in_mat: &'a Material) -> Self {
        Self {
            tris,
            out_mat,
            in_mat,
        }
    }

    /// Reference the bounding triangles.
    pub fn tris(&self) -> &Vec<Triangle> {
        &self.tris
    }

    /// Reference the outside material.
    pub fn out_mat(&self) -> &Material {
        &self.out_mat
    }

    /// Reference the inside material.
    pub fn in_mat(&self) -> &Material {
        &self.in_mat
    }
}
