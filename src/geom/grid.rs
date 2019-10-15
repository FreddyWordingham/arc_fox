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
}
