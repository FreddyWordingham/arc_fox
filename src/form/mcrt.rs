//! MCRT form structure.

use crate::{index::Resolution, json};
use contracts::post;
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Monte-Carlo Radiative Transfer input form parameters.
#[derive(Serialize, Deserialize)]
pub struct Mcrt {
    /// Resolution of the grid.
    res: [usize; 3],
    /// Grid extension in each direction.
    half_widths: Vector3<f64>,
    /// Number of photons to run.
    total_phot: u64,
    /// Number of threads to use.
    num_threads: usize,
}

impl Mcrt {
    /// Construct an example instance.
    pub fn example() -> Self {
        Self {
            res: [21, 21, 21],
            half_widths: Vector3::new(1.0, 1.0, 1.0),
            total_phot: 1_000,
            num_threads: 1,
        }
    }

    /// Get the resolution.
    pub fn res(&self) -> Resolution {
        Resolution::new(self.res[0], self.res[1], self.res[2])
    }

    /// Reference the half-width values.
    #[post(ret.iter().all(|x| *x > 0.0))]
    pub fn half_widths(&self) -> &Vector3<f64> {
        &self.half_widths
    }

    /// Get the number of threads to use.
    #[post(ret > 0)]
    pub fn num_threads(&self) -> usize {
        self.num_threads
    }
}

json!(Mcrt);
