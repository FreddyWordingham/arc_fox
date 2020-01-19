//! Substance species.

use crate::{access, ord::Named, sci::chem::SpeciesBuilder};

/// Reactive species structure.
pub struct Species {
    /// Unique name.
    name: String,
    /// Optional diffusive radius [m].
    rad: Option<f64>,
}

impl Species {
    access!(name, String);
    access!(rad, Option<f64>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(name: String, rad: Option<f64>) -> Self {
        Self { name, rad }
    }

    /// Build a new instance.
    #[inline]
    #[must_use]
    pub fn build(name: String, proto: &SpeciesBuilder) -> Self {
        Self {
            name,
            rad: *proto.rad(),
        }
    }
}

impl Named for Species {
    #[inline]
    #[must_use]
    fn name(&self) -> &str {
        &self.name
    }
}
