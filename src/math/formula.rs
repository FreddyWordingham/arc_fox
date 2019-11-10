//! Mathematical formula enumeration.

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
}
