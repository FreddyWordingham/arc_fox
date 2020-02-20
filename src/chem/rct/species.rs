//! Species implementation.

use crate::access;
use attr::json;
use std::fmt::{Display, Formatter, Result};

/// Reactive species structure.
#[json]
pub struct Species {
    /// Optional diffusive radius [m].
    rad: Option<f64>,
}

impl Species {
    access!(rad, Option<f64>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(rad: Option<f64>) -> Self {
        Self { rad }
    }
}

impl Display for Species {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        if let Some(rad) = self.rad {
            write!(fmt, "Diffusive. Radius {}nm", rad * 1.0e9)
        } else {
            write!(fmt, "Non-diffusive.")
        }
    }
}
