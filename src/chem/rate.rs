//! Univariate enumeration.

use contracts::pre;
use ndarray::Array1;

/// Univariate enumeration implementation.
/// Formulae that accept a single scalar value, and return a single scalar value.
#[derive(Debug)]
pub enum Rate {
    /// Constant value. f(cs) = C
    Zeroth(f64),
    /// Proportional to one reactant concentration.
    First(f64, usize),
    /// Proportional to two reactant concentrations.
    Second(f64, usize, usize),
}

impl Rate {
    /// Construct a new zeroth order instance.
    #[pre(k > 0.0)]
    pub fn new_zeroth_order(k: f64) -> Self {
        Rate::Zeroth(k)
    }

    /// Construct a new first order instance.
    #[pre(k > 0.0)]
    pub fn new_first_order(k: f64, ci: usize) -> Self {
        Rate::First(k, ci)
    }

    /// Construct a new second order instance.
    #[pre(k > 0.0)]
    pub fn new_second_order(k: f64, ci_0: usize, ci_1: usize) -> Self {
        Rate::Second(k, ci_0, ci_1)
    }

    /// Calculate the result of the formula.
    pub fn res(&self, concs: &Array1<f64>) -> f64 {
        match self {
            Rate::Zeroth(k) => -*k,
            Rate::First(k, ci) => -*k * concs[*ci],
            Rate::Second(k, ci_0, ci_1) => -*k * concs[*ci_0] * concs[*ci_1],
        }
    }
}
