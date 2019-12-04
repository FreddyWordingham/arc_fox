//! Light-Map structure.

use crate::sim::mcrt::Record;
use contracts::pre;
use ndarray::Array3;
use std::ops::AddAssign;

macro_rules! density_datacube {
    ($dens_func: ident, $prop: ident) => {
        /// Create a density data-cube of the lightmap's records.
        pub fn $dens_func(&self) -> Array3<f64> {
            self.recs.mapv(|rec| rec.$prop / self.cell_vol)
        }
    };
}

/// Light-Map structure implementation.
/// Stores output data from an MCRT simulation.
#[derive(Debug)]
pub struct LightMap {
    /// Record array.
    pub recs: Array3<Record>,
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
        vec![
            ("emission density", self.emission_density()),
            ("scatter density", self.scatter_density()),
            ("absorption density", self.absorption_density()),
            ("shift density", self.shift_density()),
            ("dist travelled density", self.dist_travelled_density()),
        ]
    }

    density_datacube!(emission_density, emissions);
    density_datacube!(scatter_density, scatters);
    density_datacube!(absorption_density, absorptions);
    density_datacube!(shift_density, shifts);
    density_datacube!(dist_travelled_density, dist_travelled);
}

impl AddAssign<&Self> for LightMap {
    #[pre(self.cell_vol == rhs.cell_vol)]
    fn add_assign(&mut self, rhs: &Self) {
        self.recs += &rhs.recs;
    }
}
