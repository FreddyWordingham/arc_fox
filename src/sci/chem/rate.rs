//! Rate enumeration.

use crate::ord::Name;
use attr_mac::json;
use std::fmt::{Display, Formatter, Result};

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

impl Display for Rate {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        match self {
            Self::Zeroth(k) => write!(fmt, "{}", k),
            Self::First(k, a) => write!(fmt, "{} [{}]", k, a),
            Self::Second(k, a, b) => write!(fmt, "{} [{}] [{}]", k, a, b),
            Self::Third(k, a, b, c) => write!(fmt, "{} [{}] [{}] [{}]", k, a, b, c),
            Self::Poly(k, cs) => {
                write!(fmt, "{}", k)?;
                for c in cs {
                    write!(fmt, " [{}]", c)?;
                }
                write!(fmt, "")
            }
        }
    }
}
