//! Mathematical formula enumeration.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Mathematical formulae accepting a single scalar argument.
#[derive(Debug, Serialize, Deserialize)]
pub enum Formula {
    /// Normal. f(x) = exp(- x^2 / 2) / sqrt(2 pi)
    Normal(),
    /// Constant value. f(x) = c
    Const(f64),
    /// Guassian. f(x) = exp(- (x - mu)^2 / (2 sd^2)) / sqrt(2 pi sd^2)
    Guassian(f64, f64),
}

impl Formula {
    /// Calculate the result of the formula.
    pub fn calc(&self, x: f64) -> f64 {
        match self {
            Formula::Normal() => (2.0 * PI).sqrt() * (-x.powi(2) / 2.0).exp(),
            Formula::Const(c) => *c,
            Formula::Guassian(mu, sd) => {
                (2.0 * PI * sd.powi(2)).sqrt() * (-(x - mu).powi(2) / (2.0 * sd.powi(2))).exp()
            }
        }
    }
}
