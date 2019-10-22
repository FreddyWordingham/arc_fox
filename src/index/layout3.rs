//! Three-dimensional layout structure.

use crate::phy::ThreeDimensional;
use contracts::pre;

/// Structure specifying the layout of a three-dimensional indexable object.
pub struct Layout3 {
    /// Number of indices along the X-axis.
    x: usize,
    /// Number of indices along the Y-axis.
    y: usize,
    /// Number of indices along the Z-axis.
    z: usize,
}

impl Layout3 {
    /// Construct a new instance.
    #[pre(x > 0)]
    #[pre(y > 0)]
    #[pre(z > 0)]
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    /// Calculate the total number of indices within the layout.
    pub fn total(&self) -> usize {
        self.x * self.y * self.z
    }
}

impl ThreeDimensional<usize> for Layout3 {
    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }

    fn z(&self) -> usize {
        self.z
    }
}
