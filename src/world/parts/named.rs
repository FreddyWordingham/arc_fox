//! Named trait.

/// Named trait implementation.
pub trait Named {
    /// Reference the name of the instance.
    fn name(&self) -> &str;
}

/// Determine the index of the element with a matching name.
pub fn index_of_name<T: Named>(parts: &[T], name: &str) -> usize {
    for (i, part) in parts.iter().enumerate() {
        if part.name() == name {
            return i;
        }
    }

    panic!("Name {} was not found in the given list of parts.");
}
