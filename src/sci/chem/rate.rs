//! Rate enumeration.

use attr_mac::json;

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
    Poly(f64, Vec<String>),
}
