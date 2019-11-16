//! Environment structure.

use contracts::pre;

/// Environment structure implementation.
#[derive(Debug)]
pub struct Environment {
    // Fields.
}

impl Environment {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
