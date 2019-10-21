//! Manifestable from proto-structure trait.

/// Types implementing this type can be manifested into another type.
/// Useful for setting up structures from a file in multiple stages.
pub trait Manifestable<T> {
    /// Manifest into another type.
    fn manifest(&self) -> T;
}
