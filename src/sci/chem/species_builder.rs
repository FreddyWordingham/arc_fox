//! Reaction builder structure.

use crate::access;
use attr_mac::json;

/// Species builder structure.
#[json]
pub struct SpeciesBuilder {
    /// Optional diffusive radius [m].
    rad: Option<f64>,
}

impl SpeciesBuilder {
    access!(rad, Option<f64>);
}
