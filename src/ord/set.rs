//! Set structure.

use crate::ord::Named;

/// Trait implemented for sets of named elements.
pub trait Set<T> {
    /// Determine the index of a given name in the list.
    fn index_of(&self, name: &str) -> Option<usize>;

    /// Retrieve a reference to the given name in the list.
    fn ref_of(&self, name: &str) -> Option<&T>;
}

impl<T: Named> Set<T> for &[T] {
    #[inline]
    fn index_of(&self, name: &str) -> Option<usize> {
        for (index, elem) in self.iter().enumerate() {
            if elem.name() == name {
                return Some(index);
            }
        }

        None
    }

    #[inline]
    fn ref_of(&self, name: &str) -> Option<&T> {
        for elem in self.iter() {
            if elem.name() == name {
                return Some(elem);
            }
        }

        None
    }
}
