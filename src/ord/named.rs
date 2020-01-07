//! Named trait.

/// Types implementing this trait have a unique name.
pub trait Named {
    /// Reference the name of the instance.
    fn name(&self) -> &str;
}
