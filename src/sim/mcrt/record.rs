//! Light-Map record structure.

use crate::access;
use std::ops::AddAssign;

/// Record structure implementation.
/// Stores data about a single cell during an MCRT simulation.
#[derive(Debug, Clone)]
pub struct Record {
    /// Total weight of photon emissions.
    emissions: f64,
    /// Total weight of scattering events.
    scatters: f64,
    /// Total weight of absorption events.
    absorptions: f64,
    /// Total weight of shift events.
    shifts: f64,
    /// Total distance travelled by photons.
    dist_travelled: f64,
}

impl Record {
    access!(emissions, f64);
    access!(scatters, f64);
    access!(absorptions, f64);
    access!(shifts, f64);
    access!(dist_travelled, f64);
}

impl Default for Record {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self {
            emissions: 0.0,
            scatters: 0.0,
            absorptions: 0.0,
            shifts: 0.0,
            dist_travelled: 0.0,
        }
    }
}

impl AddAssign<Self> for Record {
    fn add_assign(&mut self, rhs: Self) {
        self.emissions += rhs.emissions;
        self.scatters += rhs.scatters;
        self.absorptions += rhs.absorptions;
        self.shifts += rhs.shifts;
        self.dist_travelled += rhs.dist_travelled;
    }
}
