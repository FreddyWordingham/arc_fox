//! Record data structure.

use serde::{Deserialize, Serialize};
use std::ops::AddAssign;

/// Record structure storing cell information.
#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    /// Number of photon emissions.
    num_emissions: f64,
}

impl Record {
    /// Construct a new instance.
    pub fn new() -> Self {
        Self { num_emissions: 0.0 }
    }

    /// Get the number of photon emissions.
    pub fn num_emissions(&self) -> f64 {
        self.num_emissions
    }
}

impl AddAssign for Record {
    fn add_assign(&mut self, rhs: Self) {
        self.num_emissions += rhs.num_emissions;
    }
}
