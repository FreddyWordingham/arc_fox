//! Geometric gridding structure.

use super::Cube;
use contracts::pre;
use nalgebra::Vector3;

/// Axis-aligned regular split grid partitioning.
#[derive(Debug)]
pub struct Grid {
    /// Surface.
    surface: Cube,
    /// Number of cells in each axis.
    num_cells: [usize; 3],
    /// Cell size.
    cell_size: Vector3<f64>,
}

impl Grid {
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

    /// Reference the grid surface geometry.
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
