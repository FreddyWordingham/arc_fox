//! Numerical Range structure.

use contracts::{post, pre};
use std::f64::{INFINITY, MIN_POSITIVE, NEG_INFINITY};

/// One-dimensional inclusive Range.
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

    /// Construct a new positive range.
    pub fn new_positive() -> Self {
        Self::new(MIN_POSITIVE, INFINITY)
    }

    /// Construct a new non-positive range.
    pub fn new_non_positive() -> Self {
        Self::new(NEG_INFINITY, 0.0)
    }

    /// Construct a new negative range.
    pub fn new_negative() -> Self {
        Self::new(NEG_INFINITY, -MIN_POSITIVE)
    }

    /// Construct a new non-negative range.
    pub fn new_non_negative() -> Self {
        Self::new(0.0, INFINITY)
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
}
