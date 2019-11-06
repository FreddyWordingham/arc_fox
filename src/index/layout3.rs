//! Layout structure.

use contracts::pre;

/// Three-dimensional layout structure.
/// Used by domain grids.
#[derive(Debug, Clone)]
pub struct Layout3 {
    /// Number of indices in each dimension.
    pub nis: [usize; 3],
}

impl Layout3 {
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

    /// Get the number of splits along the x-axis.
    pub fn x(&self) -> usize {
        self.nis[0]
    }

    /// Get the number of splits along the y-axis.
    pub fn y(&self) -> usize {
        self.nis[1]
    }

    /// Get the number of splits along the z-axis.
    pub fn z(&self) -> usize {
        self.nis[2]
    }

    /// Determine if the given index is contained within the layout.
    pub fn contains(&self, index: &[usize; 3]) -> bool {
        (index[0] < self.nis[0]) && (index[1] < self.nis[1]) && (index[2] < self.nis[2])
    }
}
