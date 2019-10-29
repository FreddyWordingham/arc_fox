//! Masked ordering container structure.

use std::cmp::{Ordering, PartialEq, PartialOrd};

/// SortLabel struct used to organise values by a separate label.
#[derive(Clone, Copy)]
pub struct SortLabel<T, S> {
    /// Sorting utllabel.
    pub label: T,
    /// Stored value.
    pub value: S,
}

impl<T: Copy + PartialEq + PartialOrd, S: Copy> SortLabel<T, S> {
    /// Construct a new instance.
    pub fn new(label: T, value: S) -> Self {
        Self { label, value }
    }

    /// Destruct the object into its component label and value pair.
    pub fn components(self) -> (T, S) {
        (self.label, self.value)
    }

    /// Determine the minimum of two given SortLabels and return a copy.
    pub fn min(&self, other: &Self) -> Self {
        if self.label < other.label {
            return *self;
        }
        *other
    }

    /// Determine the maximum of two given SortLabels and return a copy.
    pub fn max(&self, other: &Self) -> Self {
        if self.label > other.label {
            return *self;
        }
        *other
    }
}

impl<T: PartialOrd, S> PartialEq for SortLabel<T, S> {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl<T: PartialOrd, S> PartialOrd for SortLabel<T, S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.label.partial_cmp(&other.label)
    }
}

impl<T: PartialOrd, S> PartialEq<T> for SortLabel<T, S> {
    fn eq(&self, other: &T) -> bool {
        self.label == *other
    }
}

impl<T: PartialOrd, S> PartialOrd<T> for SortLabel<T, S> {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.label.partial_cmp(&other)
    }
}
