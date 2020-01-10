//! Substance species.

use crate::{ord::Named, sci::chem::SpeciesBuilder};

/// Reactive species structure.
pub struct Species {
    /// Unique name.
    pub name: String,
    /// Optional diffusive radius [m].
    pub rad: Option<f64>,
}

impl Species {
    /// Construct a new instance.
    #[inline]
    pub const fn new(name: String, rad: Option<f64>) -> Self {
        Self { name, rad }
    }

    /// Build a new instance.
    #[inline]
    pub const fn build(name: String, proto: &SpeciesBuilder) -> Self {
        Self {
            name,
            rad: proto.rad,
        }
    }
}

impl Named for Species {
    #[inline]
    fn name(&self) -> &str {
        &self.name
    }
}
