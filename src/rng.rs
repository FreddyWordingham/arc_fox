//! Random number generation structures.

pub mod distribution;

pub use self::distribution::*;

use contracts::pre;
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::FRAC_PI_2;

#[pre(asym > -1.0)]
#[pre(asym < 1.0)]
#[post(ret > 0.0)]
#[post(ret < FRAC_PI_2)]
pub fn henyey_greenstein(rng: &mut ThreadRng, asym: f64) -> f64 {
    (1.0 + asym.powi(2)
        - ((1.0 - asym.powi(2)) / (1.0 + (asym * rng.gen_range(-1.0, 1.0)))).powi(2))
        / (2.0 * asym)
}
