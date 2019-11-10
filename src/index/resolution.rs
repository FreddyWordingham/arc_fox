//! Layout structure.

use super::Index;
use crate::iter::Resolution as ResolutionIter;
use contracts::pre;

/// Three-dimensional resolution structure.
/// Used by domain grids.
#[derive(Clone)]
pub struct Resolution {
    /// Number of indices in each dimension.
    arr: [usize; 3],
}

impl Resolution {
    #[pre(xi > 0)]
    #[pre(yi > 0)]
    #[pre(zi > 0)]
    pub fn new(xi: usize, yi: usize, zi: usize) -> Self {
        Self { arr: [xi, yi, zi] }
    }

    /// Reference the internal array of indices.
    pub fn arr(&self) -> &[usize; 3] {
        &self.arr
    }

    /// Get the total number of indices.
    pub fn total(&self) -> usize {
        self.arr[0] * self.arr[1] * self.arr[2]
    }

    /// Get the number of splits along the x-axis.
    pub fn x(&self) -> usize {
        self.arr[0]
    }

    /// Get the number of splits along the y-axis.
    pub fn y(&self) -> usize {
        self.arr[1]
    }

    /// Get the number of splits along the z-axis.
    pub fn z(&self) -> usize {
        self.arr[2]
    }

    /// Get the index corresponding to the n-th entry in the block.
    #[pre(n < self.total())]
    pub fn nth_index(&self, n: usize) -> Index {
        let xi = n % self.arr[0];
        let yi = ((n - xi) / self.arr[0]) % self.arr[1];
        let zi = (n - xi - (yi * self.arr[0])) / (self.arr[0] * self.arr[1]);

        Index::new(xi, yi, zi)
    }

    /// Determine if the given index is contained within the layout.
    pub fn contains(&self, index: &[usize; 3]) -> bool {
        (index[0] < self.arr[0]) && (index[1] < self.arr[1]) && (index[2] < self.arr[2])
    }

    /// Create an iterator.
    pub fn iter(&self) -> ResolutionIter {
        (&self).into_iter()
    }
}

impl IntoIterator for &Resolution {
    type Item = Index;
    type IntoIter = ResolutionIter;

    fn into_iter(self) -> Self::IntoIter {
        ResolutionIter::new(self.clone())
    }
}
