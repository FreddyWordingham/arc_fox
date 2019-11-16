//! Vacuum structure.

use contracts::pre;

/// Vacuum structure.
#[derive(Debug)]
pub struct Vacuum {
    // Fields.
}

impl Vacuum {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
