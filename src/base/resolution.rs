//! Resolution structure.

use contracts::pre;

/// Resolution structure.
#[derive(Debug)]
pub struct Resolution {
    // Fields.
}

impl Resolution {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
