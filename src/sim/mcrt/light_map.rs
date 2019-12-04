//! Light-Map structure.

use crate::sim::mcrt::Record;
use contracts::pre;
use ndarray::Array3;

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
    #[pre(res.iter().all(|x| *x > 0))]
    #[pre(cell_vol > 0.0)]
    pub fn new(res: [usize; 3], cell_vol: f64) -> Self {
        Self {
            recs: Array3::default(res),
            cell_vol,
        }
    }

    /// Generate a list of density mappings.
    pub fn generate_density_maps(&self) -> Vec<(&str, Array3<f64>)> {
        let maps = Vec::with_capacity(0);

        maps
    }
}
