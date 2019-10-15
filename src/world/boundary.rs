//! Material boundary structure.

use crate::{geom::Triangle, phy::Material};

/// Material boundary structure forming the boundary between two materials.
#[derive(Debug)]
pub struct Boundary<'a> {
    /// Bounding triangles.
    tris: Vec<Triangle>,
    /// Inside material.
    in_mat: &'a Material,
    /// Outside material.
    out_mat: &'a Material,
}

impl<'a> Boundary<'a> {
    /// Construct a new instance.
    pub fn new(tris: Vec<Triangle>, in_mat: &'a Material, out_mat: &'a Material) -> Self {
        Self {
            tris,
            in_mat,
            out_mat,
        }
    }

    /// Reference the bounding triangles.
    pub fn tris(&self) -> &Vec<Triangle> {
        &self.tris
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
