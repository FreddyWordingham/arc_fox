//! Light structure.

use contracts::pre;

/// Light structure.
#[derive(Debug)]
pub struct Light {
    // Fields.
}

impl Light {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
