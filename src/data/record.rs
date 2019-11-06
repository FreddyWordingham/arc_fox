//! Data record structure.

use crate::phys::opt::Photon;
use std::ops::{Add, AddAssign};

/// Data record.
#[derive(Debug, Clone)]
pub struct Record {
    /// Total weight of photon emissions.
    emissions: f64,
    /// Total weight of scattering events.
    scatters: f64,
    /// Total weight of absorption events.
    absorptions: f64,
}

impl Record {
    /// Construct a new instance.
    pub fn new() -> Self {
        Self {
            emissions: 0.0,
            scatters: 0.0,
            absorptions: 0.0,
        }
    }

    /// Get the number of recorded emissions.
    pub fn emissions(&self) -> f64 {
        self.emissions
    }

    /// Get the number of recorded scatterings.
    pub fn scatters(&self) -> f64 {
        self.scatters
    }

    /// Get the number of recorded absorptions.
    pub fn absorptions(&self) -> f64 {
        self.absorptions
    }

    /// Increase the number of recorded scatterings.
    pub fn increase_scatters(&mut self, phot: &Photon) {
        self.scatters += phot.weight();
    }

    /// Increase the number of recorded emissions.
    pub fn increase_emissions(&mut self, phot: &Photon) {
        self.emissions += phot.weight();
    }

    /// Increase the number of recorded absorptions.
    pub fn increase_absorptions(&mut self, phot: &Photon) {
        self.absorptions += phot.weight();
    }
}

impl Add<&Self> for Record {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self {
        Self {
            emissions: self.emissions + rhs.emissions,
            scatters: self.scatters + rhs.scatters,
            absorptions: self.absorptions + rhs.absorptions,
        }
    }
}

impl AddAssign for Record {
    fn add_assign(&mut self, rhs: Self) {
        self.emissions += rhs.emissions;
        self.scatters += rhs.scatters;
        self.absorptions += rhs.absorptions;
    }
}

impl AddAssign<&Self> for Record {
    fn add_assign(&mut self, rhs: &Self) {
        self.emissions += rhs.emissions;
        self.scatters += rhs.scatters;
        self.absorptions += rhs.absorptions;
    }
}
