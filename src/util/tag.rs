//! Tag structure.

use contracts::pre;

/// Tag structure.
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
