//! Data record structure.

use contracts::{post, pre};
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
    /// Total distance travelled by photons.
    dist_travelled: f64,
    /// Number of Raman photons created
    tot_raman: f64
}

impl Record {
    /// Construct a new instance.
    pub fn new() -> Self {
        Self {
            emissions: 0.0,
            scatters: 0.0,
            absorptions: 0.0,
            dist_travelled: 0.0,
            tot_raman: 0.0
        }
    }

    /// Get the number of recorded emissions.
    #[post(ret >= 0.0)]
    pub fn emissions(&self) -> f64 {
        self.emissions
    }

    /// Get the number of recorded scatterings.
    #[post(ret >= 0.0)]
    pub fn scatters(&self) -> f64 {
        self.scatters
    }

    /// Get the number of recorded absorptions.
    #[post(ret >= 0.0)]
    pub fn absorptions(&self) -> f64 {
        self.absorptions
    }

    /// Get the distance travelled.
    #[post(ret >= 0.0)]
    pub fn dist_travelled(&self) -> f64 {
        self.dist_travelled
    }

    /// Get the number of Raman photons.
    #[post(ret >= 0.0)]
    pub fn tot_raman(&self) -> f64 {
        self.tot_raman
    }

    /// Increase the number of recorded scatterings.
    #[pre(x > 0.0)]
    pub fn increase_scatters(&mut self, x: f64) {
        self.scatters += x;
    }

    /// Increase the number of recorded emissions.
    #[pre(x > 0.0)]
    pub fn increase_emissions(&mut self, x: f64) {
        self.emissions += x;
    }

    /// Increase the number of recorded absorptions.
    #[pre(x > 0.0)]
    pub fn increase_absorptions(&mut self, x: f64) {
        self.absorptions += x;
    }

    /// Increase the total distance travelled.
    #[pre(d > 0.0)]
    pub fn increase_dist_travelled(&mut self, d: f64) {
        self.dist_travelled += d;
    }

    /// Increase the number of Raman photons created.
    #[pre(x > 0.0)]
    pub fn increase_tot_raman(&mut self, x: f64) {
        self.tot_raman += x;
    }
}

impl Add<&Self> for Record {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self {
        Self {
            emissions: self.emissions + rhs.emissions,
            scatters: self.scatters + rhs.scatters,
            absorptions: self.absorptions + rhs.absorptions,
            dist_travelled: self.dist_travelled + rhs.dist_travelled,
            tot_raman: self.tot_raman + rhs.tot_raman,
        }
    }
}

impl AddAssign for Record {
    fn add_assign(&mut self, rhs: Self) {
        self.emissions += rhs.emissions;
        self.scatters += rhs.scatters;
        self.absorptions += rhs.absorptions;
        self.dist_travelled += rhs.dist_travelled;
        self.tot_raman += rhs.tot_raman;
    }
}

impl AddAssign<&Self> for Record {
    fn add_assign(&mut self, rhs: &Self) {
        self.emissions += rhs.emissions;
        self.scatters += rhs.scatters;
        self.absorptions += rhs.absorptions;
        self.dist_travelled += rhs.dist_travelled;
        self.tot_raman += rhs.tot_raman;
    }
}
