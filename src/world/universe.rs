//! Universe structure.

use contracts::pre;

/// Universe structure.
#[derive(Debug)]
pub struct Universe {
    // Fields.
}

impl Universe {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
