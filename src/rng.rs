//! Random number generation functions traits.

use contracts::pre;
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

#[pre(asym > -1.0)]
#[pre(asym < 1.0)]
#[post(ret > 0.0)]
#[post(ret < PI)]
pub fn henyey_greenstein(rng: &mut ThreadRng, asym: f64) -> f64 {
    if asym.abs() < 0.01 {
        return rng.gen_range(-1.0f64, 1.0).acos();
    }

    ((1.0 + asym.powi(2)
        - ((1.0 - asym.powi(2)) / (1.0 + (asym * rng.gen_range(-1.0, 1.0)))).powi(2))
        / (2.0 * asym))
        .acos()
}
