//! Cell structure.

use contracts::pre;

/// Cell structure implementation.
#[derive(Debug)]
pub struct Cell {
    // Fields.
}

impl Cell {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
