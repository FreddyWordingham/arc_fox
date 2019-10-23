//! Three-dimensional layout structure.

use crate::phy::ThreeDimensional;
use contracts::pre;

/// Structure specifying the layout of a three-dimensional indexable object.
#[derive(Debug)]
pub struct Layout3 {
    /// Number of indices along each axis.
    ni: [usize; 3],
}

impl Layout3 {
    /// Construct a new instance.
    #[pre(x > 0)]
    #[pre(y > 0)]
    #[pre(z > 0)]
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { ni: [x, y, z] }
    }

    /// Construct a new instance from a slice.
    #[pre(arr[0] > 0)]
    #[pre(arr[1] > 0)]
    #[pre(arr[2] > 0)]
    pub fn from_array(arr: [usize; 3]) -> Self {
        Self { ni: arr }
    }

    /// Create an index array.
    pub fn as_array(&self) -> [usize; 3] {
        self.ni
    }

    /// Calculate the total number of indices within the layout.
    pub fn total(&self) -> usize {
        self.ni[0] * self.ni[1] * self.ni[2]
    }
}

impl ThreeDimensional<usize> for Layout3 {
    fn x(&self) -> usize {
        self.ni[0]
    }

    fn y(&self) -> usize {
        self.ni[1]
    }

    fn z(&self) -> usize {
        self.ni[2]
    }
}
