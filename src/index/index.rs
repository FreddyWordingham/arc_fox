//! Index structure.

use crate::dim::Cartesian::{X, Y, Z};
use std::fmt::{Display, Formatter, Result};

/// Three-dimensional Index structure.
#[derive(Debug, Clone)]
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

impl Display for Index {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "[{}:{}:{}]",
            self.arr[X as usize], self.arr[Y as usize], self.arr[Z as usize]
        )
    }
}
