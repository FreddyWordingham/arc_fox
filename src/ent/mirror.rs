//! Mirror structure.

use contracts::pre;

/// Mirror structure.
#[derive(Debug)]
pub struct Mirror {
    // Fields.
}

impl Mirror {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
