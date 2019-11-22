//! Lightmap structure.

use super::Record;
use crate::{base::Resolution, file::Save};
use contracts::pre;
use ndarray::Array3;
use std::{ops::AddAssign, path::Path};

macro_rules! density_datacube {
    ($dens_func: ident, $prop_func: ident) => {
        /// Create a density data-cube of the lightmap's records.
        pub fn $dens_func(&self) -> Array3<f64> {
            self.recs.mapv(|rec| rec.$prop_func() / self.cell_vol)
        }
    };
}

/// Lightmap structure implementation.
/// Record Lightmap.
#[derive(Debug)]
pub struct Lightmap {
    /// Cell volume [m^2].
    cell_vol: f64,
    /// Record array.
    pub recs: Array3<Record>,
}

impl Lightmap {
    /// Construct a new instance.
    #[pre(cell_vol > 0.0)]
    pub fn new(cell_vol: f64, res: Resolution) -> Self {
        Self {
            cell_vol,
            recs: Array3::from_elem(*res.arr(), Record::new()),
        }
    }

    density_datacube!(emission_density, emissions);
    density_datacube!(scatter_density, scatters);
    density_datacube!(absorption_density, absorptions);
    density_datacube!(shift_density, shifts);
    density_datacube!(dist_travelled_density, dist_travelled);
}

impl Save for Lightmap {
    fn save(&self, path: &Path) {
        let data = vec![
            ("emission density", self.emission_density()),
            ("scatter density", self.scatter_density()),
            ("absorption density", self.absorption_density()),
            ("shift density", self.shift_density()),
            ("dist travelled density", self.dist_travelled_density()),
        ];

        data.save(path);
    }
}

impl AddAssign<&Lightmap> for Lightmap {
    fn add_assign(&mut self, rhs: &Self) {
        self.recs += &rhs.recs;
    }
}
