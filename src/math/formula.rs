//! Mathematical formula enumeration.

use contracts::pre;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Mathematical formulae accepting a single scalar argument.
#[derive(Serialize, Deserialize)]
pub enum Formula {
    /// Normal. f(x) = exp(- x^2 / 2) / sqrt(2 pi)
    Normal(),
    /// Constant value. f(x) = c
    Const(f64),
    /// Gaussian. f(x) = exp(- (x - mu)^2 / (2 sd^2)) / sqrt(2 pi sd^2)
    Gaussian {
        /// Average value.
        ave: f64,
        /// Standart deviation.
        sd: f64,
    },
}

impl Formula {
    /// Construct a new normal formula.
    pub fn new_normal() -> Self {
        Formula::Normal()
    }

    /// Construct a new constant formula.
    pub fn new_const(c: f64) -> Self {
        Formula::Const(c)
    }

    /// Construct a new gaussian formula.
    #[pre(sd > 0.0)]
    pub fn new_guassian(ave: f64, sd: f64) -> Self {
        Formula::Gaussian { ave, sd }
    }

    /// Calculate the result of the formula.
    pub fn calc(&self, x: f64) -> f64 {
        match self {
            Formula::Normal() => (2.0 * PI).sqrt() * (-x.powi(2) / 2.0).exp(),
            Formula::Const(c) => *c,
            Formula::Gaussian { ave, sd } => {
                (2.0 * PI * sd.powi(2)).sqrt() * (-(x - ave).powi(2) / (2.0 * sd.powi(2))).exp()
            }
        }
    }
}
