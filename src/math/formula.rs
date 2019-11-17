//! Formula enumeration.

use crate::{json, util::Range};
use serde::{Deserialize, Serialize};

/// Formula enumeration implementation.
/// Formulae that accept a single scalar value, and return a single scalar value.
#[derive(Debug, Serialize, Deserialize)]
pub enum Formula {
    /// Constant value. f(x) = C
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

json!(Formula);
