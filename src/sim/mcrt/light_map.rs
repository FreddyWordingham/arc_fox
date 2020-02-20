//! Light-Map structure.

use crate::{access, file::Save, sim::Record};
use ndarray::Array3;
use std::{ops::AddAssign, path::Path};

/// Light-Map structure implementation.
/// Stores output data from an MCRT simulation.
#[derive(Debug)]
pub struct LightMap {
    /// Record array.
    recs: Array3<Record>,
    /// Cell volume [m^2].
    cell_vol: f64,
}

impl LightMap {
    access!(recs, recs_mut, Array3<Record>);
    access!(cell_vol, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 3], cell_vol: f64) -> Self {
        Self {
            recs: Array3::default(res),
            cell_vol,
        }
    }
}

impl AddAssign<&Self> for LightMap {
    fn add_assign(&mut self, rhs: &Self) {
        self.recs += &rhs.recs;
    }
}

impl Save for LightMap {
    fn save(&self, out_dir: &Path) {
        self.recs
            .map(|r| r.emissions() / self.cell_vol)
            .save(&out_dir.join("emission_dens.nc"));
        self.recs
            .map(|r| r.scatters() / self.cell_vol)
            .save(&out_dir.join("scat_dens.nc"));
        self.recs
            .map(|r| r.absorptions() / self.cell_vol)
            .save(&out_dir.join("abs_dens.nc"));
        self.recs
            .map(|r| r.shifts() / self.cell_vol)
            .save(&out_dir.join("shift_dens.nc"));
        self.recs
            .map(|r| r.dist_travelled() / self.cell_vol)
            .save(&out_dir.join("dist_travelled_dens.nc"));
        self.recs
            .map(|r| r.det_raman() / self.cell_vol)
            .save(&out_dir.join("det_raman_dens.nc"));
    }
}
