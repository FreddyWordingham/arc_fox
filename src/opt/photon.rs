//! Photon structure.

use contracts::pre;

/// Photon structure implementation.
#[derive(Debug)]
pub struct Photon {
    // Fields.
}

impl Photon {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
