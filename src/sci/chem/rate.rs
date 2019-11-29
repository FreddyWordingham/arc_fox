//! Rate enumeration.

// use contracts::pre;
use ndarray::Array1;

/// Univariate enumeration implementation.
/// Formulae that accept a single scalar value, and return a single scalar value.
#[derive(Debug)]
pub enum Rate {
    /// Niladic function. f(cs) = k;
    ZerothOrder(f64),
    /// Monadic. f(cs) = k[A];
    FirstOrder(f64, usize),
    /// Dyadic. f(cs) = k[A][B];
    SecondOrder(f64, usize, usize),
    /// Triadic. f(cs) = k[A][B][C];
    ThirdOrder(f64, usize, usize, usize),
    /// Polyadic. f(cs) = prod(k[n]);
    PolyOrder(f64, Array1<usize>),
}

impl Rate {
    /// Calculate the current rate.
    pub fn res(&self, concs: &Array1<f64>) -> f64 {
        match self {
            Rate::ZerothOrder(k) => -k,
            Rate::FirstOrder(k, a) => -k * concs[*a],
            Rate::SecondOrder(k, a, b) => -k * concs[*a] * concs[*b],
            Rate::ThirdOrder(k, a, b, c) => -k * concs[*a] * concs[*b] * concs[*c],
            Rate::PolyOrder(k, ss) => {
                let p: f64 = ss.iter().map(|a| concs[*a]).product();
                -k * p
            }
        }
    }
}
