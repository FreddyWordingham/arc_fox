//! CCD detector structure.

use crate::{access, sci::math::geom::Rectangle};
use ndarray::Array2;

/// CCD detector used to form images.
pub struct Ccd {
    /// Detection surface.
    surf: Rectangle,
    /// Image data.
    data: Array2<f64>,
}

impl Ccd {
    access!(surf, Rectangle);
    access!(data, Array2<f64>);

    /// Construct a new instance.
    pub fn new(surf: Rectangle, res: [usize; 2]) -> Self {
        Self {
            surf,
            data: Array2::zeros(res),
        }
    }

    /// Determine the resolution.
    pub fn res(&self) -> [usize; 2] {
        let px = self.data.shape()[0];
        let py = self.data.shape()[1];

        [px, py]
    }
}
