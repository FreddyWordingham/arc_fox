//! Layout structure.

use contracts::pre;

/// Layout structure.
/// Used by domain grids.
pub struct Layout {
    /// Number of indices in each dimension.
    nis: [usize; 3],
}

impl Layout {
    #[pre(xi > 0)]
    #[pre(yi > 0)]
    #[pre(zi > 0)]
    pub fn new(xi: usize, yi: usize, zi: usize) -> Self {
        Self { nis: [xi, yi, zi] }
    }

    /// Get the total number of indices.
    pub fn total_indices(&self) -> usize {
        self.nis[0] * self.nis[1] * self.nis[2]
    }

    /// Reference the internal array.
    pub fn nis(&self) -> &[usize; 3] {
        &self.nis
    }

    /// Get the number of indices in the x-dimension.
    pub fn x(&self) -> usize {
        self.nis[0]
    }

    /// Get the number of indices in the y-dimension.
    pub fn y(&self) -> usize {
        self.nis[1]
    }

    /// Get the number of indices in the z-dimension.
    pub fn z(&self) -> usize {
        self.nis[2]
    }
}
