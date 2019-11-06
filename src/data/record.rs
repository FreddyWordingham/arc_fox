//! Data record structure.

use contracts::pre;
use std::ops::{Add, AddAssign};

/// Data record.
#[derive(Debug, Clone)]
pub struct Record {
    /// Total weight of photon emissions.
    emissions: f64,
}

impl Record {
    /// Construct a new instance.
    pub fn new() -> Self {
        Self { emissions: 0.0 }
    }

    /// Increase the number of recorded emissions.
    #[pre(w > 0.0)]
    pub fn increase_emissions(&mut self, w: f64) {
        self.emissions += w;
    }
}

impl Add<&Self> for Record {
    type Output = Self;

    fn add(self, _rhs: &Self) -> Self {
        Self::new()
    }
}

impl AddAssign for Record {
    fn add_assign(&mut self, _rhs: Self) {}
}

impl AddAssign<&Self> for Record {
    fn add_assign(&mut self, _rhs: &Self) {}
}
