//! Histogram structure.

use contracts::pre;

/// Histogram structure implementation.
#[derive(Debug)]
pub struct Histogram {
    // Fields.
}

impl Histogram {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}