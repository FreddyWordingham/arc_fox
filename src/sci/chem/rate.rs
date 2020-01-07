//! Rate enumeration.

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
    /// Construct a new zeroth-order instance.
    #[inline]
    pub const fn new_zeroth(k: f64) -> Self {
        Self::Zeroth { 0: k }
    }

    /// Construct a new first-order instance.
    #[inline]
    pub fn new_first(k: f64, a: usize) -> Self {
        Self::First { 0: k, 1: a }
    }

    /// Construct a new second-order instance.
    #[inline]
    pub fn new_second(k: f64, a: usize, b: usize) -> Self {
        Self::Second { 0: k, 1: a, 2: b }
    }

    /// Construct a new third-order instance.
    #[inline]
    pub fn new_third(k: f64, a: usize, b: usize, c: usize) -> Self {
        Self::Third {
            0: k,
            1: a,
            2: b,
            3: c,
        }
    }

    /// Construct a new nth-order instance.
    #[inline]
    pub fn new_poly(k: f64, cs: Array1<usize>) -> Self {
        Self::Poly { 0: k, 1: cs }
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
