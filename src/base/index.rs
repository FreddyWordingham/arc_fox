//! Index structure.

use crate::list::dimension::Cartesian::{X, Y, Z};

/// Index structure implementation.
#[derive(Debug, Clone)]
pub struct Index {
    /// Number of indices in each dimension.
    arr: [usize; 3],
}

impl Index {
    /// Construct a new instance.
    pub fn new(xi: usize, yi: usize, zi: usize) -> Self {
        Self { arr: [xi, yi, zi] }
    }

    /// Get the x component.
    pub fn x(&self) -> usize {
        self.arr[X as usize]
    }

    /// Get the y component.
    pub fn y(&self) -> usize {
        self.arr[Y as usize]
    }

    /// Get the z component.
    pub fn z(&self) -> usize {
        self.arr[Z as usize]
    }

    /// Reference the underlying array.
    pub fn arr(&self) -> &[usize; 3] {
        &self.arr
    }
}
