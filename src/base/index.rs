//! Index structure.

use contracts::pre;

/// Index structure implementation.
#[derive(Debug)]
pub struct Index {
    // Fields.
}

impl Index {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
