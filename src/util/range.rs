//! Range structure.

use contracts::pre;

/// Range structure.
#[derive(Debug)]
pub struct Range {
    // Fields.
}

impl Range {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
