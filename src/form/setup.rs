//! Setup structure.

use contracts::pre;

/// Setup structure implementation.
#[derive(Debug)]
pub struct Setup {
    // Fields.
}

impl Setup {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
