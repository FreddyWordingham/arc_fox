//! Random number generation distribution enumeration.

use rand::rngs::ThreadRng;

/// Distribution enumeration.
pub enum Distribution {
    /// Constant number generation.
    Const(f64),
}

impl Distribution {
    /// Construct a new constant distribution.
    pub fn new_const(c: f64) -> Distribution {
        Distribution::Const(c)
    }

    /// Generate a new value.
    pub fn gen(&self, _rng: &mut ThreadRng) -> f64 {
        match self {
            Distribution::Const(c) => *c,
        }
    }
}
