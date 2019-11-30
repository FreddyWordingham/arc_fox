//! Rate enumeration.

use ndarray::Array1;
use serde::{Deserialize, Serialize};

/// Univariate enumeration implementation.
/// Formulae that accept a single scalar value, and return a single scalar value.
#[derive(Debug, Deserialize, Serialize)]
pub enum Rate {
    /// Niladic function. f(cs) = k;
    Zeroth(f64),
    /// Monadic. f(cs) = k[A];
    First(f64, usize),
    /// Dyadic. f(cs) = k[A][B];
    Second(f64, usize, usize),
    /// Triadic. f(cs) = k[A][B][C];
    Third(f64, usize, usize, usize),
    /// Polyadic. f(cs) = prod(k[n]);
    Poly(f64, Array1<usize>),
}

impl Rate {
    /// Calculate the current rate.
    pub fn res(&self, concs: &Array1<f64>) -> f64 {
        match self {
            Self::Zeroth(k) => -k,
            Self::First(k, a) => -k * concs[*a],
            Self::Second(k, a, b) => -k * concs[*a] * concs[*b],
            Self::Third(k, a, b, c) => -k * concs[*a] * concs[*b] * concs[*c],
            Self::Poly(k, ss) => {
                let p: f64 = ss.iter().map(|a| concs[*a]).product();
                -k * p
            }
        }
    }
}
