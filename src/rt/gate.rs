//! Gate structure.

use contracts::pre;

/// Gate structure.
#[derive(Debug)]
pub struct Gate {
    // Fields.
}

impl Gate {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
