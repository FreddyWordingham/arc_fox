//! Setup form structure.

use crate::json;
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Monte-Carlo Radiative Transfer input form parameters.
#[derive(Serialize, Deserialize)]
pub struct Mcrt {
    /// Grid extension in each direction.
    half_widths: Vector3<f64>,
}

impl Mcrt {
    /// Construct an example instance.
    pub fn example() -> Self {
        Self {
            half_widths: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

json!(Mcrt);
