//! Mesh structure.

use contracts::pre;

/// Mesh structure.
#[derive(Debug)]
pub struct Mesh {
    // Fields.
}

impl Mesh {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
