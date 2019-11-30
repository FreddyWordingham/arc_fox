//! Formula enumeration.

use super::super::Range;
use serde::{Deserialize, Serialize};

/// Formula enumeration implementation.
/// Formulae that accept a single scalar value, and return a single scalar value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Formula {
    /// Constant value. f(x) = C
    Const(f64),
    /// Bifurcated value. f(x) = x <= D ?: A : B
    Bifur(f64, f64, f64),
}

impl Formula {
    /// Construct a new constant formula.
    pub fn new_const(c: f64) -> Self {
        Formula::Const(c)
    }

    /// Construct a new constant formula.
    pub fn new_bifur(s: f64, a: f64, b: f64) -> Self {
        Formula::Bifur(s, a, b)
    }

    /// Calculate the result of the formula.
    pub fn res(&self, x: f64) -> f64 {
        match self {
            Formula::Const(c) => *c,
            Formula::Bifur(s, a, b) => {
                if x <= *s {
                    *a
                } else {
                    *b
                }
            }
        }
    }

    /// Determine the minimum result value within the given range.
    pub fn min(&self, range: Range) -> f64 {
        match self {
            Formula::Const(c) => *c,
            Formula::Bifur(s, a, b) => {
                if range.max() <= *s {
                    return *a;
                } else if range.min() > *s {
                    return *b;
                }
                a.min(*b)
            }
        }
    }

    /// Determine the maximum result value within the given range.
    pub fn max(&self, range: Range) -> f64 {
        match self {
            Formula::Const(c) => *c,
            Formula::Bifur(s, a, b) => {
                if range.max() <= *s {
                    return *a;
                } else if range.min() > *s {
                    return *b;
                }
                a.max(*b)
            }
        }
    }
}
