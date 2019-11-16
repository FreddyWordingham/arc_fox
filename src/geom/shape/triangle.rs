//! Triangle structure.

use contracts::pre;

/// Triangle structure implementation.
#[derive(Debug)]
pub struct Triangle {
    // Fields.
}

impl Triangle {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
