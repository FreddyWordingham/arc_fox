//! Masked ordering container structure.

use std::cmp::{Ordering, PartialEq, PartialOrd};

/// Container struct used to organise values by a separate label.
#[derive(Clone, Copy)]
pub struct Container<T, S> {
    /// Sorting label.
    label: T,
    /// Stored value.
    value: S,
}

impl<T: Copy + PartialEq + PartialOrd, S: Copy> Container<T, S> {
    /// Construct a new instance.
    pub fn new(label: T, value: S) -> Self {
        Self { label, value }
    }

    /// Get the sorting label.
    pub fn label(&self) -> T {
        self.label
    }

    /// Get the contained value.
    pub fn value(&self) -> S {
        self.value
    }

    /// Destruct the object into its component label and value pair.
    pub fn separate(self) -> (T, S) {
        (self.label, self.value)
    }

    /// Determine the minimum of two given containers and return a copy.
    pub fn min(&self, other: &Self) -> Self {
        if self.label < other.label {
            return *self;
        }
        *other
    }

    /// Determine the maximum of two given containers and return a copy.
    pub fn max(&self, other: &Self) -> Self {
        if self.label > other.label {
            return *self;
        }
        *other
    }
}

impl<T: PartialOrd, S> PartialEq for Container<T, S> {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl<T: PartialOrd, S> PartialOrd for Container<T, S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.label.partial_cmp(&other.label)
    }
}

impl<T: PartialOrd, S> PartialEq<T> for Container<T, S> {
    fn eq(&self, other: &T) -> bool {
        self.label == *other
    }
}

impl<T: PartialOrd, S> PartialOrd<T> for Container<T, S> {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.label.partial_cmp(&other)
    }
}
