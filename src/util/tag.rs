//! Tag structure.

use std::cmp::{Ordering, PartialEq, PartialOrd};

/// Tag structure implementation.
/// Sorts according to the label.
#[derive(Debug, Clone, Copy)]
pub struct Tag<T, S> {
    /// Sorting label.
    pub label: T,
    /// Stored value.
    pub value: S,
}

impl<T: Copy + PartialEq + PartialOrd, S: Copy> Tag<T, S> {
    /// Construct a new instance.
    pub fn new(label: T, value: S) -> Self {
        Self { label, value }
    }

    /// Destruct the object into its component label and value pair.
    pub fn components(self) -> (T, S) {
        (self.label, self.value)
    }

    /// Determine the minimum and return a copy.
    pub fn min(&self, other: &Self) -> Self {
        if self.label < other.label {
            return *self;
        }
        *other
    }

    /// Determine the maximum and return a copy.
    pub fn max(&self, other: &Self) -> Self {
        if self.label > other.label {
            return *self;
        }
        *other
    }
}

impl<T: PartialOrd, S> PartialEq for Tag<T, S> {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl<T: PartialOrd, S> PartialOrd for Tag<T, S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.label.partial_cmp(&other.label)
    }
}

impl<T: PartialOrd, S> PartialEq<T> for Tag<T, S> {
    fn eq(&self, other: &T) -> bool {
        self.label == *other
    }
}

impl<T: PartialOrd, S> PartialOrd<T> for Tag<T, S> {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.label.partial_cmp(&other)
    }
}
