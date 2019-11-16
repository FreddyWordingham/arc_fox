//! Interface structure.

use contracts::pre;

/// Interface structure implementation.
#[derive(Debug)]
pub struct Interface {
    // Fields.
}

impl Interface {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
