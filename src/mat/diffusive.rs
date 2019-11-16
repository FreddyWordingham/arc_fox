//! Diffusive structure.

use contracts::pre;

/// Diffusive structure implementation.
#[derive(Debug)]
pub struct Diffusive {
    // Fields.
}

impl Diffusive {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
