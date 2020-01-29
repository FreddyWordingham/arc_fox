//! Interface implementation.

use crate::{access, geom::Mesh, sim::Material};

/// Material interface structure.
pub struct Interface<'a> {
    /// Surface mesh.
    surf: Mesh,
    /// Inside material.
    in_mat: &'a Material,
    /// Outside material.
    out_mat: &'a Material,
}

impl<'a> Interface<'a> {
    access!(surf, Mesh);
    access!(in_mat, Material);
    access!(out_mat, Material);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(surf: Mesh, in_mat: &'a Material, out_mat: &'a Material) -> Self {
        Self {
            surf,
            in_mat,
            out_mat,
        }
    }
}
