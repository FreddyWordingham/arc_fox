//! Record structure.

use contracts::pre;
use std::ops::{Add, AddAssign};

/// Record structure implementation.
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
    /// Number of photons skipped.
    tot_skip: f64,
    /// Number of Raman photons detected.
    det_raman: f64
}

impl Record {
    /// Construct a new instance.
    pub fn new() -> Self {
        Self {
            emissions: 0.0,
            scatters: 0.0,
            absorptions: 0.0,
            shifts: 0.0,
            dist_travelled: 0.0,
            tot_skip: 0.0,
            det_raman: 0.0
        }
    }

    /// Get the total weight of photon emissions.
    pub fn emissions(&self) -> f64 {
        self.emissions
    }

    /// Get the total weight of scattering events.
    pub fn scatters(&self) -> f64 {
        self.scatters
    }

    /// Get the total weight of absorption events.
    pub fn absorptions(&self) -> f64 {
        self.absorptions
    }

    /// Get the total weight of shift events.
    pub fn shifts(&self) -> f64 {
        self.shifts
    }

    /// Get the total distance travelled by photons.
    pub fn dist_travelled(&self) -> f64 {
        self.dist_travelled
    }

    ///Get the number of skipped photons.
    pub fn tot_skip(&self) -> f64{
        self.tot_skip
    }

    ///Get the total weight of detected Raman photons.
    pub fn det_raman(&self) -> f64{
        self.det_raman
    }

    #[pre(x > 0.0)]
    pub fn increase_emissions(&mut self, x: f64) {
        self.emissions += x;
    }

    #[pre(x > 0.0)]
    pub fn increase_scatters(&mut self, x: f64) {
        self.scatters += x;
    }

    #[pre(x > 0.0)]
    pub fn increase_absorptions(&mut self, x: f64) {
        self.absorptions += x;
    }

    #[pre(x > 0.0)]
    pub fn increase_shifts(&mut self, x: f64) {
        self.shifts += x;
    }

    #[pre(x > 0.0)]
    pub fn increase_dist_travelled(&mut self, x: f64) {
        self.dist_travelled += x;
    }

    /// Increase the number of photons skipped.
    #[pre(x > 0.0)]
    pub fn increase_tot_skip(&mut self, x: f64) {
        self.tot_skip += x;
    }

    /// Increase the number of Raman photons detected.
    #[pre(x > 0.0)]
    pub fn increase_det_raman(&mut self, x: f64) {
        self.det_raman += x;
    }
}

impl Add<&Self> for Record {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self {
        Self {
            emissions: self.emissions + rhs.emissions,
            scatters: self.scatters + rhs.scatters,
            absorptions: self.absorptions + rhs.absorptions,
            shifts: self.shifts + rhs.shifts,
            dist_travelled: self.dist_travelled + rhs.dist_travelled,
            tot_skip: self.tot_skip + rhs.tot_skip,
            det_raman: self.det_raman + rhs.det_raman
        }
    }
}

impl AddAssign for Record {
    fn add_assign(&mut self, rhs: Self) {
        self.emissions += rhs.emissions;
        self.scatters += rhs.scatters;
        self.absorptions += rhs.absorptions;
        self.shifts += rhs.shifts;
        self.dist_travelled += rhs.dist_travelled;
        self.tot_skip += rhs.tot_skip;
        self.det_raman += rhs.det_raman
    }
}

impl AddAssign<&Self> for Record {
    fn add_assign(&mut self, rhs: &Self) {
        self.emissions += rhs.emissions;
        self.scatters += rhs.scatters;
        self.absorptions += rhs.absorptions;
        self.shifts += rhs.shifts;
        self.dist_travelled += rhs.dist_travelled;
        self.tot_skip += rhs.tot_skip;
        self.det_raman += rhs.det_raman
    }
}
