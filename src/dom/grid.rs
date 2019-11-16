//! Grid structure.

use contracts::pre;

/// Grid structure.
#[derive(Debug)]
pub struct Grid {
    // Fields.
}

impl Grid {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
