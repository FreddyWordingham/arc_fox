//! Aabb structure.

use contracts::pre;

/// Aabb structure.
#[derive(Debug)]
pub struct Aabb {
    // Fields.
}

impl Aabb {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}
