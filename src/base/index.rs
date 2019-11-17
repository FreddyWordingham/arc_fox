//! Index structure.

use crate::list::dimension::Cartesian::{X, Y, Z};
use contracts::{post, pre};

/// Index structure implementation.
#[derive(Debug, Clone)]
pub struct Index {
    /// Number of indices in each dimension.
    arr: [usize; 3],
}

impl Index {
    /// Construct a new instance.
    #[pre(xi > 0)]
    #[pre(yi > 0)]
    #[pre(zi > 0)]
    pub fn new(xi: usize, yi: usize, zi: usize) -> Self {
        Self { arr: [xi, yi, zi] }
    }

    /// Get the x component.
    #[post(ret > 0)]
    pub fn x(&self) -> usize {
        self.arr[X as usize]
    }

    /// Get the y component.
    #[post(ret > 0)]
    pub fn y(&self) -> usize {
        self.arr[Y as usize]
    }

    /// Get the z component.
    #[post(ret > 0)]
    pub fn z(&self) -> usize {
        self.arr[Z as usize]
    }

    /// Reference the underlying array.
    pub fn arr(&self) -> &[usize; 3] {
        &self.arr
    }
}
