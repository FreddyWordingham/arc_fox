//! World domain partitioning structure.

use crate::geom::Cube;
use contracts::pre;
use nalgebra::Vector3;
use std::fmt::{Display, Formatter, Result};

/// Domain with regular split grid partitioning.
#[derive(Debug)]
pub struct Domain {
    /// Boundary.
    boundary: Cube,
    /// Number of cells in each axis.
    num_cells: [usize; 3],
    /// Cell size.
    cell_size: Vector3<f64>,
}

impl Domain {
    /// Construct a new instance.
    #[pre(num_cells[0] > 0)]
    #[pre(num_cells[1] > 0)]
    #[pre(num_cells[2] > 0)]
    pub fn new(boundary: Cube, num_cells: [usize; 3]) -> Self {
        let mut cell_size = boundary.widths();
        cell_size.x /= num_cells[0] as f64;
        cell_size.y /= num_cells[1] as f64;
        cell_size.z /= num_cells[2] as f64;

        Self {
            boundary,
            num_cells,
            cell_size,
        }
    }

    /// Reference the boundary geometry.
    pub fn boundary(&self) -> &Cube {
        &self.boundary
    }

    /// Reference the number of cells.
    pub fn num_cells(&self) -> &[usize; 3] {
        &self.num_cells
    }

    /// Create a shape tuple of the domain.
    pub fn shape(&self) -> (usize, usize, usize) {
        (self.num_cells[0], self.num_cells[1], self.num_cells[2])
    }

    /// Reference the cell size.
    pub fn cell_size(&self) -> &Vector3<f64> {
        &self.cell_size
    }

    /// Construct the boundary of a cell at a given index.
    #[pre(index[0] < self.num_cells[0])]
    #[pre(index[1] < self.num_cells[1])]
    #[pre(index[2] < self.num_cells[2])]
    pub fn cell_boundary(&self, index: [usize; 3]) -> Cube {
        let mut min = self.boundary.mins().clone();
        min.x += self.cell_size.x * index[0] as f64;
        min.y += self.cell_size.y * index[1] as f64;
        min.z += self.cell_size.z * index[2] as f64;

        Cube::new(min, min + self.cell_size)
    }
}

impl Display for Domain {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mins = self.boundary.mins();
        let maxs = self.boundary.maxs();

        writeln!(f, "Mins     : {},\t{},\t{}", mins.x, mins.y, mins.z)?;
        writeln!(f, "Maxs     : {},\t{},\t{}", maxs.x, maxs.y, maxs.z)?;
        writeln!(
            f,
            "Cells    : {} - {} - {}",
            self.num_cells[0], self.num_cells[1], self.num_cells[2]
        )?;
        write!(
            f,
            "Num cells: {}",
            self.num_cells[0] * self.num_cells[1] * self.num_cells[2]
        )
    }
}
