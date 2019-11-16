//! Molecule structure.

use contracts::pre;

/// Molecule structure.
#[derive(Debug)]
pub struct Molecule {
    // Fields.
}

impl Molecule {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
