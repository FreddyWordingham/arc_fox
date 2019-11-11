//! Tag trait.

/// Types implementing the tag trait can be requested for id.
pub trait Tag {
    fn id(&self) -> &str;
}
