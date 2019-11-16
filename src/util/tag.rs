//! Tag structure.

use contracts::pre;

/// Tag structure implementation.
#[derive(Debug)]
pub struct Tag {
    // Fields.
}

impl Tag {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
