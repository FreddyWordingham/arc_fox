//! Archive structure.

use contracts::pre;

/// Archive structure implementation.
#[derive(Debug)]
pub struct Archive {
    // Fields.
}

impl Archive {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
