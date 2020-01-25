//! State building structure.

use crate::access;
use attr_mac::json;

/// Reaction builder structure.
#[json]
#[derive(Clone)]
pub struct StateBuilder {
    /// Initial state of species concentration.
    concs: Option<Vec<(String, f64)>>,
    /// Initial state of species source/sink terms.
    sources: Option<Vec<(String, f64)>>,
}

impl StateBuilder {
    access!(concs, Option<Vec<(String, f64)>>);
    access!(sources, Option<Vec<(String, f64)>>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        concs: Option<Vec<(String, f64)>>,
        sources: Option<Vec<(String, f64)>>,
    ) -> Self {
        Self { concs, sources }
    }
}
