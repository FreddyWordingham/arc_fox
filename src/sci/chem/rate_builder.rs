//! Rate-Builder enumeration.

use ndarray::Array1;
use serde::{Deserialize, Serialize};

/// Rate-Builder structure implementation.
/// Used to build Rates.
#[derive(Debug, Deserialize, Serialize)]
pub enum RateBuilder {
    /// Niladic function. f(cs) = k;
    ZerothOrder(f64),
    /// Monadic. f(cs) = k[A];
    FirstOrder(f64, String),
    /// Dyadic. f(cs) = k[A][B];
    SecondOrder(f64, String, String),
    /// Triadic. f(cs) = k[A][B][C];
    ThirdOrder(f64, String, String, String),
    /// Polyadic. f(cs) = prod(k[n]);
    PolyOrder(f64, Array1<String>),
}
