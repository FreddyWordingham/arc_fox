//! Isogram structure.

use contracts::pre;

/// Isogram structure.
#[derive(Debug)]
pub struct Isogram {
    // Fields.
}

impl Isogram {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
