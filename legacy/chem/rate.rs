//! Rate enumeration.

use crate::sci::chem::State;
use attr_mac::json;
use ndarray::Array1;
use std::collections::BTreeMap;

/// Rates that accept a single scalar value, and return a single scalar value.
#[json]
pub enum Rate {
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

impl Rate {
    /// Calculate the current rate.
    #[inline]
    #[must_use]
    pub fn calc(&self, state: &State) -> f64 {
        // TODO: Change to Species alias.
        match self {
            Self::Zeroth(k) => -k,
            Self::First(k, a) => {
                -k * state
                    .get(a)
                    .expect("Could not get concentration from index.")
                    .conc()
            }
            Self::Second(k, a, b) => {
                -k * state
                    .get(a)
                    .expect("Could not get concentration from index.")
                    .conc()
                    * state
                        .get(b)
                        .expect("Could not get concentration from index.")
                        .conc()
            }
            Self::Third(k, a, b, c) => {
                -k * state
                    .get(a)
                    .expect("Could not get concentration from index.")
                    .conc()
                    * state
                        .get(b)
                        .expect("Could not get concentration from index.")
                        .conc()
                    * state
                        .get(c)
                        .expect("Could not get concentration from index.")
                        .conc()
            }
            Self::Poly(k, cs) => {
                let p: f64 = cs
                    .iter()
                    .map(|n| {
                        state
                            .get(n)
                            .expect("Could not get concentration from index.")
                            .conc()
                    })
                    .product();
                -k * p
            }
        }
    }
}
