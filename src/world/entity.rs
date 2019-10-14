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
