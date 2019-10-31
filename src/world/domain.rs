//! World domain structure.

use super::{Cell, Record};
use crate::{geom::Aabb, util::progress::bar};
use nalgebra::Point3;
use ndarray::Array3;

/// World domain structure.
/// All simulation is contained within the boundary of the domain.
#[derive(Debug)]
pub struct Domain {
    /// Number of splits along each axis.
    num_cells: [usize; 3],
    /// Boundary.
    boundary: Aabb,
    /// Array of cells.
    cells: Array3<Cell>,
    /// Array of records.
    recs: Array3<Record>,
}

impl Domain {
    /// Construct a new instance.
    pub fn new(num_cells: [usize; 3], boundary: Aabb) -> Self {
        let total_cells = num_cells[0] * num_cells[1] * num_cells[2];
        let mut cells = Vec::with_capacity(total_cells);

        let mut cell_size = boundary.widths();
        for i in 0..3 {
            cell_size[i] /= num_cells[i] as f64;
        }

        let bar = bar("Constructing cells", total_cells as u64);
        for xi in 0..num_cells[0] {
            let min_x = boundary.mins().x + (cell_size.x * xi as f64);
            for yi in 0..num_cells[1] {
                let min_y = boundary.mins().y + (cell_size.y * yi as f64);
                for zi in 0..num_cells[2] {
                    bar.inc(1);

                    let min_z = boundary.mins().z + (cell_size.z * zi as f64);
                    let min = Point3::new(min_x, min_y, min_z);
                    let max = min + cell_size;

                    cells.push(Cell::new(Aabb::new(min, max)));
                }
            }
        }
        bar.finish_with_message(&format!("{} cells constructed.", total_cells));

        Self {
            num_cells,
            boundary,
            cells: Array3::from_shape_vec(num_cells, cells).unwrap(),
            recs: Array3::from_elem(num_cells, Record::new()),
        }
    }

    /// Reference the number of cells.
    pub fn num_cells(&self) -> &[usize; 3] {
        &self.num_cells
    }

    /// Reference the boundary.
    pub fn boundary(&self) -> &Aabb {
        &self.boundary
    }

    /// Reference the array of cells.
    pub fn cells(&self) -> &Array3<Cell> {
        &self.cells
    }

    /// Reference the array of records.
    pub fn recs(&self) -> &Array3<Record> {
        &self.recs
    }
}
