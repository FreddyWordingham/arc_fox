//! Histogram data reduction structure.

use crate::util::Range;
use ndarray::Array1;

/// Static range, constant bin width, Histogram.
pub struct Histogram {
    /// Domain of values.
    range: Range,
    /// Width of each bin.
    bin_width: f64,
    /// Bin data.
    bins: Array1<f64>,
}

impl Histogram {
    /// Construct a new instance
    #[inline]
    #[must_use]
    pub fn new(range: Range, num_bins: usize) -> Self {
        let bin_width = range.width() / num_bins as f64;

        Self {
            range,
            bin_width,
            bins: Array1::zeros(num_bins),
        }
    }
}
