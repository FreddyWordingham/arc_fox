//! Range structure.

use crate::json;
use contracts::{post, pre};
use serde::{Deserialize, Serialize};
use std::f64::{INFINITY, MIN_POSITIVE, NEG_INFINITY};

/// Range structure implementation.
/// One-dimensional inclusive Range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    /// Minimum bound.
    min: f64,
    /// Maximum bound.
    max: f64,
}

impl Range {
    /// Construct a new instance.
    #[pre(min < max)]
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// Construct an infinite range.
    pub fn new_infinite() -> Self {
        Self::new(NEG_INFINITY, INFINITY)
    }

    /// Get the minimum bound.
    pub fn min(&self) -> f64 {
        self.min
    }

    /// Get the maximum bound.
    pub fn max(&self) -> f64 {
        self.max
    }

    /// Calculate the width of the Range.
    #[post(ret > 0.0)]
    pub fn width(&self) -> f64 {
        self.max - self.min
    }

    /// Determine if a value is contained within the Range.
    pub fn contains(&self, x: f64) -> bool {
        if x < self.min || x > self.max {
            return false;
        }

        true
    }

    /// Determine if the Range intersects with another given Range.
    pub fn intersect(&self, other: &Self) -> bool {
        if self.max < other.min || other.max < self.min {
            return false;
        }

        true
    }

    /// Form a range of overlapping values.
    pub fn overlap(&self, other: &Self) -> Option<Self> {
        if !self.intersect(other) {
            return None;
        }

        let min = self.min.max(other.min);
        let max = self.max.min(other.max);

        Some(Self::new(min, max))
    }

    /// Determine the index corresponding to a given point in the range.
    #[pre(self.contains(x))]
    #[post(ret < n)]
    pub fn find_index(&self, x: f64, n: usize) -> usize {
        (((x - self.min) / self.width()).min(1.0 - MIN_POSITIVE) * n as f64).floor() as usize
    }
}

json!(Range);
