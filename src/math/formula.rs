//! Mathematical formula enumeration.

use serde::{Deserialize, Serialize};

/// Mathematical formulae accepting a single scalar argument.
#[derive(Debug, Serialize, Deserialize)]
pub enum Formula {
    /// No-op. = x
    X,
    /// Constant value. = c
    Const(f64),
}

impl Formula {
    /// Calculate the result of the formula.
    pub fn calc(&self, x: f64) -> f64 {
        match self {
            Formula::X => x,
            Formula::Const(c) => *c,
        }
    }
}
