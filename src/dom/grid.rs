//! Grid structure.

use super::{Aabb, Cell};
use crate::{
    data::Archive,
    index::{bin, Layout3},
    util::progress::bar,
    world::EntMap,
};
use contracts::pre;
use nalgebra::Point3;
use ndarray::Array3;

/// Domain cell grid.
#[derive(Debug)]
pub struct Grid<'a> {
    /// Layout.
    layout: Layout3,
    /// Boundary.
    aabb: Aabb,
    /// Cells.
    cells: Array3<Cell<'a>>,
}

impl<'a> Grid<'a> {
    /// Construct a new instance.
    pub fn new(layout: Layout3, aabb: Aabb, ent_map: &'a EntMap) -> Self {
        let mut cells = Vec::with_capacity(layout.total_indices());
        let mut cell_size = aabb.widths();
        for (w, n) in cell_size.iter_mut().zip(&layout.nis) {
            *w /= *n as f64;
        }

        let aabb_mins = aabb.mins();
        let bar = bar("Constructing cells", layout.total_indices() as u64);
        for xi in 0..layout.x() {
            let min_x = aabb_mins.x + (cell_size.x * xi as f64);
            for yi in 0..layout.y() {
                let min_y = aabb_mins.y + (cell_size.y * yi as f64);
                for zi in 0..layout.z() {
                    let min_z = aabb_mins.z + (cell_size.z * zi as f64);

                    bar.inc(1);

                    let mins = Point3::new(min_x, min_y, min_z);
                    let maxs = mins + cell_size;

                    cells.push(Cell::new(Aabb::new(mins, maxs), ent_map, &aabb));
                }
            }
        }
        bar.finish_with_message(&format!("{} cells constructed.", layout.total_indices()));

        let cells = Array3::from_shape_vec(layout.nis, cells).unwrap();

        Self {
            layout,
            aabb,
            cells,
        }
    }

    /// Reference the layout.
    pub fn layout(&self) -> &Layout3 {
        &self.layout
    }

    /// Reference the boundary.
    pub fn aabb(&self) -> &Aabb {
        &self.aabb
    }

    /// Reference the cells.
    pub fn cells(&self) -> &Array3<Cell<'a>> {
        &self.cells
    }

    /// Reference a cell from a given position.
    #[pre(self.aabb.contains(pos))]
    pub fn get_cell(&self, pos: &Point3<f64>) -> &Cell<'a> {
        &self.cells[bin::point3(pos, &self.aabb, &self.layout)]
    }

    /// Add an archive into the cells.
    #[pre(self.cells.shape() == archive.recs.shape())]
    pub fn add_archive(&mut self, archive: Archive) {
        for (cell, rec) in self.cells.iter_mut().zip(archive.recs.iter()) {
            cell.add_record(rec);
        }
    }
}
