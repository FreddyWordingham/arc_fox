//! Rate  builder enumeration.

use ndarray::Array1;
use serde::{Deserialize, Serialize};

/// Builds reaction rates.
#[derive(Debug, Deserialize, Serialize)]
pub enum RateBuilder {
    /// Niladic function. f(cs) = k
    Zeroth(f64),
    /// Monadic. f(cs) = k[A]
    First(f64, String),
    /// Dyadic. f(cs) = k[A][B]
    Second(f64, String, String),
    /// Triadic. f(cs) = k[A][B][C]
    Third(f64, String, String, String),
    /// Polyadic. f(cs) = prod(k[n])
    Poly(f64, Array1<String>),
}

impl RateBuilder {
    /// List all the catalysts of the reaction.
    #[inline]
    pub fn catalysts(&self) -> Vec<String> {
        match self {
            Self::Zeroth(_) => vec![],
            Self::First(_, a) => vec![a.clone()],
            Self::Second(_, a, b) => vec![a.clone(), b.clone()],
            Self::Third(_, a, b, c) => vec![a.clone(), b.clone(), c.clone()],
            Self::Poly(_, cs) => cs.as_slice().expect("Invalid poly rate.").to_vec(),
        }
    }
}
