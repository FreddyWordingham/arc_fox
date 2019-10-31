//! World domain structure.

use super::{Cell, EntMap, Record};
use crate::{geom::Aabb, util::progress::bar};
use nalgebra::Point3;
use ndarray::Array3;

/// World domain structure.
/// All simulation is contained within the boundary of the domain.
pub struct Domain<'a> {
    /// Number of splits along each axis.
    shape: [usize; 3],
    /// Boundary.
    boundary: Aabb,
    /// Array of cells.
    cells: Array3<Cell<'a>>,
    /// Array of records.
    recs: Array3<Record>,
}

impl<'a> Domain<'a> {
    /// Construct a new instance.
    pub fn new(shape: [usize; 3], boundary: Aabb, ent_map: &'a EntMap<'a>) -> Self {
        let total_cells = shape[0] * shape[1] * shape[2];
        let mut cells = Vec::with_capacity(total_cells);

        let mut cell_size = boundary.widths();
        for i in 0..3 {
            cell_size[i] /= shape[i] as f64;
        }

        let bar = bar("Constructing cells", total_cells as u64);
        for xi in 0..shape[0] {
            let min_x = boundary.mins().x + (cell_size.x * xi as f64);
            for yi in 0..shape[1] {
                let min_y = boundary.mins().y + (cell_size.y * yi as f64);
                for zi in 0..shape[2] {
                    bar.inc(1);

                    let min_z = boundary.mins().z + (cell_size.z * zi as f64);
                    let min = Point3::new(min_x, min_y, min_z);
                    let max = min + cell_size;

                    cells.push(Cell::new(Aabb::new(min, max), ent_map));
                }
            }
        }
        bar.finish_with_message(&format!("{} cells constructed.", total_cells));

        Self {
            shape,
            boundary,
            cells: Array3::from_shape_vec(shape, cells).unwrap(),
            recs: Array3::from_elem(shape, Record::new()),
        }
    }

    /// Reference the number of cells.
    pub fn shape(&self) -> &[usize; 3] {
        &self.shape
    }

    /// Calculate the total number of cells.
    pub fn total_cells(&self) -> usize {
        self.shape[0] * self.shape[1] * self.shape[2]
    }

    /// Reference the boundary.
    pub fn boundary(&self) -> &Aabb {
        &self.boundary
    }

    /// Reference the array of cells.
    pub fn cells(&self) -> &Array3<Cell<'a>> {
        &self.cells
    }

    /// Reference the array of records.
    pub fn recs(&self) -> &Array3<Record> {
        &self.recs
    }
}
