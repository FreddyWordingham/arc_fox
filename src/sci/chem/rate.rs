//! Rate enumeration.

use crate::ord::Name;
use attr_mac::json;

/// Rates that accept a single scalar value, and return a single scalar value.
#[json]
pub enum Rate {
    /// Niladic function. f(cs) = k
    Zeroth(f64),
    /// Monadic. f(cs) = k[A]
    First(f64, Name),
    /// Dyadic. f(cs) = k[A][B]
    Second(f64, Name, Name),
    /// Triadic. f(cs) = k[A][B][C]
    Third(f64, Name, Name, Name),
    /// Polyadic. f(cs) = prod(k[n])
    Poly(f64, Vec<Name>),
}
