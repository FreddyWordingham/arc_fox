//! World domain partitioning structure.

use crate::geom::Cube;
use contracts::pre;
use nalgebra::Vector3;
use std::fmt::{Display, Formatter, Result};

/// Domain with regular split grid partitioning.
#[derive(Debug)]
pub struct Domain {
    /// Surface.
    surface: Cube,
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
    pub fn new(surface: Cube, num_cells: [usize; 3]) -> Self {
        let mut cell_size = surface.widths();
        cell_size.x /= num_cells[0] as f64;
        cell_size.y /= num_cells[1] as f64;
        cell_size.z /= num_cells[2] as f64;

        Self {
            surface,
            num_cells,
            cell_size,
        }
    }

    /// Reference the surface geometry.
    pub fn surface(&self) -> &Cube {
        &self.surface
    }

    /// Reference the number of cells.
    pub fn num_cells(&self) -> &[usize; 3] {
        &self.num_cells
    }

    /// Reference the cell size.
    pub fn cell_size(&self) -> &Vector3<f64> {
        &self.cell_size
    }

    /// Construct the surface of a cell at a given index.
    #[pre(index[0] < self.num_cells[0])]
    #[pre(index[1] < self.num_cells[1])]
    #[pre(index[2] < self.num_cells[2])]
    pub fn cell_surface(&self, index: [usize; 3]) -> Cube {
        let mut min = self.surface.mins().clone();
        min.x += self.cell_size.x * index[0] as f64;
        min.y += self.cell_size.y * index[1] as f64;
        min.z += self.cell_size.z * index[2] as f64;

        Cube::new(min, min + self.cell_size)
    }
}

impl Display for Domain {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mins = self.surface.mins();
        let maxs = self.surface.maxs();

        writeln!(f, "Mins     : {},\t{},\t{}", mins.x, mins.y, mins.z)?;
        writeln!(f, "Maxs     : {},\t{},\t{}", maxs.x, maxs.y, maxs.z)?;
        writeln!(f, "Cells    : {} - {} - {}", self.num_cells[0], self.num_cells[1], self.num_cells[2])?;
        write!(f,   "Num cells: {}", self.num_cells[0] * self.num_cells[1] * self.num_cells[2])
    }
}
