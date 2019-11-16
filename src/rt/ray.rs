//! Ray structure.

use contracts::pre;

/// Ray structure.
#[derive(Debug)]
pub struct Ray {
    // Fields.
}

impl Ray {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
