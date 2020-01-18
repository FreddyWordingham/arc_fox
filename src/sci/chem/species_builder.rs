//! Reaction builder structure.

use attr_mac::json;

/// Species builder structure.
#[json]
pub struct SpeciesBuilder {
    /// Optional diffusive radius [m].
    pub rad: Option<f64>,
}
