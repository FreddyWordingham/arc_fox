//! Resolution structure.

use crate::list::dimension::Cartesian::{X, Y, Z};
use contracts::{post, pre};
use serde::{Deserialize, Serialize};

/// Resolution structure implementation.
#[derive(Debug, Deserialize, Serialize)]
pub struct Resolution {
    /// Number of indices in each dimension.
    pub arr: [usize; 3],
}

impl Resolution {
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

    /// Get the total number of indices.
    #[post(ret > 0)]
    pub fn total(&self) -> usize {
        self.arr[X as usize] * self.arr[Y as usize] * self.arr[Z as usize]
    }
}
