//! Interface implementation.

use crate::{access, dom::Name};
use attr::json;
use std::fmt::{Display, Formatter, Result};

/// Material interface structure.
#[json]
pub struct Interface {
    /// Surface mesh.
    surf: Name,
    /// Inside material.
    in_mat: Name,
    /// Outside material.
    out_mat: Name,
}

impl<'a> Interface {
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

impl Display for Interface {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(
            fmt,
            "surf: {}\t{} :| {}",
            self.surf, self.in_mat, self.out_mat
        )
    }
}
