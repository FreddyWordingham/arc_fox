//! Build trait.

/// Types implementing this trait can be built from prototype.
pub trait Build<T, S> {
    /// Build final type from prototype.
    fn build(proto: T) -> S;
}
