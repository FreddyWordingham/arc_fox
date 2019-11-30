//! Rate-Builder enumeration.

use ndarray::Array1;
use serde::{Deserialize, Serialize};

/// Rate-Builder structure implementation.
/// Used to build Rates.
#[derive(Debug, Deserialize, Serialize)]
pub enum RateBuilder {
    /// Niladic function. f(cs) = k;
    Zeroth(f64),
    /// Monadic. f(cs) = k[A];
    First(f64, String),
    /// Dyadic. f(cs) = k[A][B];
    Second(f64, String, String),
    /// Triadic. f(cs) = k[A][B][C];
    Third(f64, String, String, String),
    /// Polyadic. f(cs) = prod(k[n]);
    Poly(f64, Array1<String>),
}
