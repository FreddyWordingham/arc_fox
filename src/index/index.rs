//! Index structure.

use contracts::pre;

/// Three-dimensional Index structure.
pub struct Index {
    /// Number of indices in each dimension.
    arr: [usize; 3],
}

impl Index {
    pub fn new(xi: usize, yi: usize, zi: usize) -> Self {
        Self { arr: [xi, yi, zi] }
    }

    /// Reference the internal array of indices.
    pub fn arr(&self) -> &[usize; 3] {
        &self.arr
    }

    /// Get the x component.
    pub fn x(&self) -> usize {
        self.arr[0]
    }

    /// Get the y component.
    pub fn y(&self) -> usize {
        self.arr[1]
    }

    /// Get the z component.
    pub fn z(&self) -> usize {
        self.arr[2]
    }
}
