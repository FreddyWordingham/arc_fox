//! Interface structure.

use crate::{access, ord::Name};

/// Material interface structure.
pub struct Interface {
    /// Surface mesh id.
    surf: Name,
    /// Inside material id.
    in_mat: Name,
    /// Outside material id.
    out_mat: Name,
}

impl Interface {
    access!(surf, Name);
    access!(in_mat, Name);
    access!(out_mat, Name);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(surf: Name, in_mat: Name, out_mat: Name) -> Self {
        Self {
            surf,
            in_mat,
            out_mat,
        }
    }
}
