//! Three-dimensional layout structure.

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
}
