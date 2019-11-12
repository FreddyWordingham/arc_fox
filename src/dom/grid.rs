//! Grid structure.

use super::Cell;
use crate::{data::Archive, geom::Aabb, index::Resolution, util::progress::bar, world::Entity};
use contracts::pre;
use nalgebra::Vector3;
use ndarray::Array3;

/// Domain cell grid.
#[derive(Debug)]
pub struct Grid<'a> {
    /// Boundary.
    aabb: Aabb,
    /// Resolution.
    res: Resolution,
    /// Cells.
    cells: Array3<Cell<'a>>,
}

impl<'a> Grid<'a> {
    /// Construct a new instance.
    #[pre(!ents.is_empty())]
    pub fn new(aabb: Aabb, res: Resolution, ents: &'a Vec<Entity>) -> Self {
        let mut cell_size = aabb.widths();
        for (bw, n) in cell_size.iter_mut().zip(res.arr.iter()) {
            *bw /= *n as f64;
        }

        let mut cells = Vec::with_capacity(res.total());
        let bar = bar("Constructing cells", res.total() as u64);
        for index in res.iter() {
            bar.inc(1);

            let mins = aabb.mins()
                + Vector3::new(
                    cell_size.x * index.x() as f64,
                    cell_size.y * index.y() as f64,
                    cell_size.z * index.z() as f64,
                );
            let maxs = mins + cell_size;

            cells.push(Cell::new(&aabb, ents, Aabb::new(mins, maxs)));
        }
        bar.finish_with_message(&format!("{} cells constructed.", res.total()));

        let cells = Array3::from_shape_vec(res.arr, cells).unwrap();

        Self { aabb, res, cells }
    }

    /// Reference the boundary.
    pub fn aabb(&self) -> &Aabb {
        &self.aabb
    }

    /// Reference the grid resolution.
    pub fn res(&self) -> &Resolution {
        &self.res
    }

    /// Reference the cells.
    pub fn cells(&self) -> &Array3<Cell<'a>> {
        &self.cells
    }

    /// Add an archive into the cells.
    #[pre(self.cells.shape() == archive.recs.shape())]
    pub fn add_archive(&mut self, archive: Archive) {
        for (cell, rec) in self.cells.iter_mut().zip(archive.recs.iter()) {
            cell.add_record(rec);
        }
    }
}
