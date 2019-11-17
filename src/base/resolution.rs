//! Resolution structure.

use super::Index;
use crate::list::dimension::Cartesian::{X, Y, Z};
use contracts::{post, pre};
use serde::{Deserialize, Serialize};

/// Resolution structure implementation.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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

    /// Get the index corresponding to the n-th entry in the block.
    #[pre(n < self.total())]
    pub fn nth_index(&self, n: usize) -> Index {
        let zi = n % self.arr[X as usize];
        let yi = ((n - zi) / self.arr[X as usize]) % self.arr[Y as usize];
        let xi =
            (n - zi - (yi * self.arr[X as usize])) / (self.arr[X as usize] * self.arr[Y as usize]);

        Index::new(xi, yi, zi)
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

/// Index resolution iterator struct.
#[derive(Debug, Clone)]
pub struct ResolutionIter {
    /// Current index.
    n: usize,
    /// Resolution limit.
    res: Resolution,
}

impl ResolutionIter {
    /// Construct a new instance.
    #[pre(res.total() > 0)]
    pub fn new(res: Resolution) -> Self {
        Self { n: 0, res }
    }
}

impl Iterator for ResolutionIter {
    type Item = Index;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n >= self.res.total() {
            return None;
        }

        let index = self.res.nth_index(self.n);
        self.n += 1;

        Some(index)
    }
}
