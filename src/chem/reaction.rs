//! Reaction structure.

use crate::world::Identity;
use contracts::pre;

/// Chemical reaction structure.
pub struct Reaction {
    /// Identification string.
    id: String,
    /// Rate.
    rate: f64,
}

impl Reaction {
    #[pre(!id.is_empty())]
    #[pre(rate > 0.0)]
    /// Construct a new instance.
    pub fn new(id: String, rate: f64) -> Self {
        Self { id, rate }
    }
}

impl Identity for Reaction {
    fn id(&self) -> &str {
        &self.id
    }
}
