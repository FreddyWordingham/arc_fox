//! Reaction structure.

use contracts::pre;

/// Reaction structure implementation.
#[derive(Debug)]
pub struct Reaction {
    // Fields.
}

impl Reaction {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
