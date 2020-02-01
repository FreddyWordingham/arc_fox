//! Light-Map structure.

use crate::sim::Record;
use ndarray::Array3;
use std::ops::AddAssign;

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
