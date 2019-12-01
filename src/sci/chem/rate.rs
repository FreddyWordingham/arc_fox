//! Rate enumeration.

use crate::{
    sci::chem::{RateBuilder, Species},
    world::parts::index_of_name,
};
use contracts::pre;
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
    /// Construct a new zeroth-order instance.
    #[pre(k > 0.0)]
    pub fn new_zeroth(k: f64) -> Self {
        Self::Zeroth(k)
    }

    /// Construct a new first-order instance.
    #[pre(k > 0.0)]
    pub fn new_first(k: f64, a: usize) -> Self {
        Self::First(k, a)
    }

    /// Construct a new second-order instance.
    #[pre(k > 0.0)]
    pub fn new_second(k: f64, a: usize, b: usize) -> Self {
        Self::Second(k, a, b)
    }

    /// Construct a new third-order instance.
    #[pre(k > 0.0)]
    pub fn new_third(k: f64, a: usize, b: usize, c: usize) -> Self {
        Self::Third(k, a, b, c)
    }

    /// Construct a new third-order instance.
    #[pre(k > 0.0)]
    pub fn new_poly(k: f64, is: Array1<usize>) -> Self {
        Self::Poly(k, is)
    }

    /// Build a new instance.
    pub fn build(builder: RateBuilder, species: &[Species]) -> Self {
        match builder {
            RateBuilder::Zeroth(k) => Self::new_zeroth(k),
            RateBuilder::First(k, a) => Self::new_first(k, index_of_name(&species, &a)),
            RateBuilder::Second(k, a, b) => {
                Self::new_second(k, index_of_name(&species, &a), index_of_name(&species, &b))
            }
            RateBuilder::Third(k, a, b, c) => Self::new_third(
                k,
                index_of_name(&species, &a),
                index_of_name(&species, &b),
                index_of_name(&species, &c),
            ),
            RateBuilder::Poly(k, is) => {
                Self::new_poly(k, is.iter().map(|a| index_of_name(&species, &a)).collect())
            }
        }
    }

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
