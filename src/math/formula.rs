//! Mathematical formula enumeration.

use crate::dom::Range;
use serde::{Deserialize, Serialize};

/// Mathematical formulae accepting a single scalar argument.
#[derive(Debug, Serialize, Deserialize)]
pub enum Formula {
    /// Constant value. f(x) = c
    Const(f64),
}

impl Formula {
    /// Construct a new constant formula.
    pub fn new_const(c: f64) -> Self {
        Formula::Const(c)
    }

    /// Calculate the result of the formula.
    pub fn res(&self, _x: f64) -> f64 {
        match self {
            Formula::Const(c) => *c,
        }
    }

    /// Determine the minimum result value within the given range.
    pub fn min(&self, _range: Range) -> f64 {
        match self {
            Formula::Const(c) => *c,
        }
    }

    /// Determine the maximum result value within the given range.
    pub fn max(&self, _range: Range) -> f64 {
        match self {
            Formula::Const(c) => *c,
        }
    }
}
