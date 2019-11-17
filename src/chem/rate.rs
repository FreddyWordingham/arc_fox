//! Univariate enumeration.

use ndarray::Array1;

/// Univariate enumeration implementation.
/// Formulae that accept a single scalar value, and return a single scalar value.
#[derive(Debug)]
pub enum Rate {
    /// Constant value. f(cs) = C
    Const(f64),
}

impl Rate {
    /// Construct a new constant formula.
    pub fn new_const(c: f64) -> Self {
        Rate::Const(c)
    }

    /// Calculate the result of the formula.
    pub fn res(&self, _concs: Array1<f64>) -> f64 {
        match self {
            Rate::Const(c) => *c,
        }
    }
}
