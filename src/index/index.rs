//! Index structure.

use crate::dim::Cartesian::{X, Y, Z};

/// Three-dimensional Index structure.
#[derive(Clone)]
pub struct Index {
    /// Number of indices in each dimension.
    pub arr: [usize; 3],
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
}
