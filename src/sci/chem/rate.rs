//! Rate enumeration.

use crate::{
    ord::Set,
    sci::chem::{RateBuilder, Species},
};
use ndarray::Array1;

/// Rates that accept a single scalar value, and return a single scalar value.
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
    /// Build an instance.
    pub fn build(proto: RateBuilder, species: &[Species]) -> Self {
        match proto {
            RateBuilder::Zeroth(k) => Rate::Zeroth(k),
            RateBuilder::First(k, a) => Rate::First(
                k,
                species
                    .index_of(&a)
                    .expect("Could not locate rate species in known species list."),
            ),
            RateBuilder::Second(k, a, b) => Rate::Second(
                k,
                species
                    .index_of(&a)
                    .expect("Could not locate rate species in known species list."),
                species
                    .index_of(&b)
                    .expect("Could not locate rate species in known species list."),
            ),
            RateBuilder::Third(k, a, b, c) => Rate::Third(
                k,
                species
                    .index_of(&a)
                    .expect("Could not locate rate species in known species list."),
                species
                    .index_of(&b)
                    .expect("Could not locate rate species in known species list."),
                species
                    .index_of(&c)
                    .expect("Could not locate rate species in known species list."),
            ),
            RateBuilder::Poly(k, cs) => Rate::Poly(
                k,
                cs.iter()
                    .map(|c| {
                        species
                            .index_of(c)
                            .expect("Could not locate rate species in known species list.")
                    })
                    .collect(),
            ),
        }
    }

    /// Calculate the current rate.
    #[inline]
    pub fn res(&self, concs: &Array1<f64>) -> f64 {
        match self {
            Self::Zeroth(k) => -k,
            Self::First(k, a) => {
                -k * concs
                    .get(*a)
                    .expect("Could not get concentration from index.")
            }
            Self::Second(k, a, b) => {
                -k * concs
                    .get(*a)
                    .expect("Could not get concentration from index.")
                    * concs
                        .get(*b)
                        .expect("Could not get concentration from index.")
            }
            Self::Third(k, a, b, c) => {
                -k * concs
                    .get(*a)
                    .expect("Could not get concentration from index.")
                    * concs
                        .get(*b)
                        .expect("Could not get concentration from index.")
                    * concs
                        .get(*c)
                        .expect("Could not get concentration from index.")
            }
            Self::Poly(k, cs) => {
                let p: f64 = cs
                    .iter()
                    .map(|n| {
                        concs
                            .get(*n)
                            .expect("Could not get concentration from index.")
                    })
                    .product();
                -k * p
            }
        }
    }
}
