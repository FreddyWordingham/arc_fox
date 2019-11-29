//! Named trait.

/// Named trait implementation.
pub trait Named {
    /// Reference the name of the instance.
    fn name(&self) -> &str;
}
