//! Mathematical formula enumeration.

use std::f64::consts::PI;
use serde::{Deserialize, Serialize};

/// Mathematical formulae accepting a single scalar argument.
#[derive(Serialize, Deserialize)]
pub enum Formula {
    /// Normal. f(x) = exp(- x^2 / 2) / sqrt(2 pi)
    Normal(),
    /// Constant value. f(x) = c
    Const(f64),
    /// Gaussian. f(x) = exp(- (x - mu)^2 / (2 sd^2)) / sqrt(2 pi sd^2)
    Gaussian(f64, f64),
}

impl Formula {
    /// Calculate the result of the formula.
    pub fn calc(&self, x: f64) -> f64 {
        match self {
            Formula::Normal() => (2.0 * PI).sqrt() * (-x.powi(2) / 2.0).exp(),
            Formula::Const(c) => *c,
            Formula::Gaussian(mu, sd) => {
                (2.0 * PI * sd.powi(2)).sqrt() * (-(x - mu).powi(2) / (2.0 * sd.powi(2))).exp()
            }
        }
    }
}
