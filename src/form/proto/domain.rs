//! Domain proto-structure.

use crate::{geom::Cube, world::Domain as NeoDomain, index::Layout3};
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
    /// Layout of the cells.
    layout: [usize; 3],
}

impl Domain {
    /// Construct a new instance.
    #[pre(mins < maxs)]
    #[pre(layout[0] > 0)]
    #[pre(layout[1] > 0)]
    #[pre(layout[2] > 0)]
    pub fn new(mins: [f64; 3], maxs: [f64; 3], layout: [usize; 3]) -> Self {
        Self { mins, maxs, layout }
    }

    /// Manifest the proto-domain into a full domain structure.
    pub fn manifest(&self) -> NeoDomain {
        NeoDomain::new(
            Cube::new(
                Point3::from_slice(&self.mins),
                Point3::from_slice(&self.maxs),
            ),
            Layout3::from_slice(&self.layout),
        )
    }
}
