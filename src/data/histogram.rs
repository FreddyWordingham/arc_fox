//! Histogram data reduction structure.

use crate::{access, file::io::Save, util::Range};
use ndarray::Array1;
use std::{fs::File, io::Write, path::Path};

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
    access!(range, Range);
    access!(bin_width, f64);
    access!(bins, Array1<f64>);

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

    /// Find the corresponding index of the bin data for a given value.
    fn find_index(&self, x: f64) -> usize {
        (((x - self.range.min()) / self.range.width()) * self.bins.len() as f64).floor() as usize
    }

    /// Increment the bin corresponding to x by unity.
    pub fn collect(&mut self, x: f64) {
        let index = self.find_index(x);
        self.bins[index] += 1.0;
    }

    /// Increment the bin corresponding to x by the given weight.
    pub fn collect_weight(&mut self, x: f64, weight: f64) {
        let index = self.find_index(x);
        self.bins[index] += weight;
    }

    /// Increment the bin corresponding to x by unity if the value of x is within the value range.
    pub fn try_collect(&mut self, x: f64) {
        if !self.range.contains(x) {
            return;
        }

        self.collect(x);
    }

    /// Increment the bin corresponding to x by the given weight if the value of x is within the value range.
    pub fn try_collect_weight(&mut self, x: f64, weight: f64) {
        if !self.range.contains(x) {
            return;
        }

        self.collect_weight(x, weight);
    }
}

impl Save for Histogram {
    fn save(&self, path: &Path) {
        let mut file = File::create(path).unwrap();

        for (iter, value) in self.bins.iter().enumerate() {
            let x = ((iter as f64 + 0.5) * self.bin_width()) + self.range.min();
            writeln!(file, "{:>31}, {:>31}", x, value).unwrap();
        }
    }
}
