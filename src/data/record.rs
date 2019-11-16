//! Record structure.

use contracts::pre;

/// Record structure.
#[derive(Debug)]
pub struct Record {
    // Fields.
}

impl Record {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
