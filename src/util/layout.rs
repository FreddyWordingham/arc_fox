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
    pub fn new(
            xi: usize,
            yi: usize,
            zi: usize,
    ) -> Self {
        Self {
            nis: [xi, yi, zi]
        }
    }

    /// Get the total number of indices.
    pub fn total_indices(&self) -> usize {
        self.nis[0] * self.nis[1] * self.nis[2]
    }
}
