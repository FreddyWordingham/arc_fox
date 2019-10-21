//! Domain proto-structure.

use crate::{geom::Cube, world::Domain as NeoDomain};
use contracts::pre;
use nalgebra::Point3;
use serde::{Deserialize, Serialize};

/// Proto-domain structure used to manifest domain structures.
#[derive(Debug, Deserialize, Serialize)]
pub struct Domain {
    /// Minimum bound.
    mins: [f64; 3],
    /// Maximum bound.
    maxs: [f64; 3],
    /// Number of cells.
    num_cells: [usize; 3],
}

impl Domain {
    /// Construct a new instance.
    #[pre(mins < maxs)]
    #[pre(num_cells[0] > 0)]
    #[pre(num_cells[1] > 0)]
    #[pre(num_cells[2] > 0)]
    pub fn new(mins: [f64; 3], maxs: [f64; 3], num_cells: [usize; 3]) -> Self {
        Self {
            mins,
            maxs,
            num_cells,
        }
    }

    /// Manifest the proto-domain into a full domain structure.
    pub fn manifest(&self) -> NeoDomain {
        NeoDomain::new(
            Cube::new(
                Point3::from_slice(&self.mins),
                Point3::from_slice(&self.maxs),
            ),
            self.num_cells,
        )
    }
}
