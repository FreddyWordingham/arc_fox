//! Verse implementation.

use attr::json;

/// Verse construction form.
#[json]
pub struct Verse {
    /// List of interfaces.
    interfaces: Vec<String>,
}

impl Verse {
    // pub fn form() -> Type {}
}
