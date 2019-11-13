//! Layout structure.

use super::Index;
use crate::{
    dim::Cartesian::{X, Y, Z},
    iter::Resolution as ResolutionIter,
};
use contracts::pre;
use std::fmt::{Display, Formatter, Result};

/// Three-dimensional resolution structure.
/// Used by domain grids.
#[derive(Debug, Clone)]
pub struct Resolution {
    /// Number of indices in each dimension.
    pub arr: [usize; 3],
}

impl Resolution {
    #[pre(xi > 0)]
    #[pre(yi > 0)]
    #[pre(zi > 0)]
    pub fn new(xi: usize, yi: usize, zi: usize) -> Self {
        Self { arr: [xi, yi, zi] }
    }

    /// Get the total number of indices.
    pub fn total(&self) -> usize {
        self.arr[X as usize] * self.arr[Y as usize] * self.arr[Z as usize]
    }

    /// Get the number of splits along the x-axis.
    pub fn x(&self) -> usize {
        self.arr[X as usize]
    }

    /// Get the number of splits along the y-axis.
    pub fn y(&self) -> usize {
        self.arr[Y as usize]
    }

    /// Get the number of splits along the z-axis.
    pub fn z(&self) -> usize {
        self.arr[Z as usize]
    }

    /// Get the index corresponding to the n-th entry in the block.
    #[pre(n < self.total())]
    pub fn nth_index(&self, n: usize) -> Index {
        let zi = n % self.arr[X as usize];
        let yi = ((n - zi) / self.arr[X as usize]) % self.arr[Y as usize];
        let xi =
            (n - zi - (yi * self.arr[X as usize])) / (self.arr[X as usize] * self.arr[Y as usize]);

        Index::new(xi, yi, zi)
    }

    /// Determine if the given index is contained within the layout.
    pub fn contains(&self, index: &[usize; 3]) -> bool {
        (index[X as usize] < self.arr[X as usize])
            && (index[Y as usize] < self.arr[Y as usize])
            && (index[Z as usize] < self.arr[Z as usize])
    }

    /// Create an iterator.
    pub fn iter(&self) -> ResolutionIter {
        (&self).into_iter()
    }
}

impl Display for Resolution {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "[ {} : {} : {}]",
            self.arr[X as usize], self.arr[Y as usize], self.arr[Z as usize]
        )
    }
}

impl IntoIterator for &Resolution {
    type Item = Index;
    type IntoIter = ResolutionIter;

    fn into_iter(self) -> Self::IntoIter {
        ResolutionIter::new(self.clone())
    }
}
