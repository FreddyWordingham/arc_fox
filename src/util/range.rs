//! Numerical Range structure.

use contracts::pre;
use serde::{Deserialize, Serialize};
use std::f64::{INFINITY, NEG_INFINITY};

/// One-dimensional inclusive Range.
#[derive(Debug, Serialize, Deserialize)]
pub struct Range {
    /// Minimum bound.
    min: f64,
    /// Maximum bound.
    max: f64,
}

impl Range {
    /// Construct a new Range.
    #[pre(min < max)]
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// Construct an infinite Range.
    pub fn new_infinite() -> Self {
        Self {
            min: NEG_INFINITY,
            max: INFINITY,
        }
    }

    /// Retrieve the minimum bound of the Range.
    pub fn min(&self) -> f64 {
        self.min
    }

    /// Retrieve the maximum bound of the Range.
    pub fn max(&self) -> f64 {
        self.max
    }

    /// Calculate the width of the Range.
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

    /// From a range of overlapping values.
    pub fn overlap(&self, other: &Self) -> Option<Self> {
        if !self.intersect(other) {
            return None;
        }

        let min = self.min.max(other.min);
        let max = self.max.min(other.max);

        Some(Self::new(min, max))
    }
}
