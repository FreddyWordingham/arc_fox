//! Grid geometry input.

use crate::geom::{Cube, Grid as gGrid};
use contracts::pre;
use nalgebra::Point3;
use serde::{Deserialize, Serialize};

/// Grid setup information.
#[derive(Debug, Deserialize, Serialize)]
pub struct Grid {
    /// Minimum bound.
    mins: [f64; 3],
    /// Maximum bound.
    maxs: [f64; 3],
    /// Number of cells.
    num_cells: [usize; 3],
}

impl Grid {
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

    /// Manifest into a completed structure.
    pub fn manifest<'a>(&self) -> gGrid {
        gGrid::new(
            Cube::new(
                Point3::from_slice(&self.mins),
                Point3::from_slice(&self.maxs),
            ),
            self.num_cells,
        )
    }

    /// Reference the mesh string.
    pub fn mins(&self) -> &[f64; 3] {
        &self.mins
    }

    /// Reference the maximum bound.
    pub fn maxs(&self) -> &[f64; 3] {
        &self.maxs
    }

    /// Reference the number of cells.
    pub fn num_cells(&self) -> &[usize; 3] {
        &self.num_cells
    }
}
