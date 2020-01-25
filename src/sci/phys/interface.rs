//! Interface structure.

use crate::{access, sci::math::geom::shape::Mesh};

/// Material interface structure.
pub struct Interface {
    /// Surface mesh.
    surf: Mesh,
    /// Inside material id.
    in_mat: String,
    /// Outside material id.
    out_mat: String,
}

impl Interface {
    access!(surf, Mesh);
    access!(in_mat, String);
    access!(out_mat, String);

    /// Construct a new instance.
    pub fn new(surf: Mesh, in_mat: String, out_mat: String) -> Self {
        Self {
            surf,
            in_mat,
            out_mat,
        }
    }
}
