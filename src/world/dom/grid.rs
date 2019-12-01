//! Grid structure.

use crate::{
    sci::math::shape::Aabb,
    util::{
        list::dimension::Cartesian::{X, Y, Z},
        progress::{ParallelBar, SerialBar},
    },
    world::dom::Cell,
};
use contracts::pre;
use nalgebra::Vector3;
use ndarray::Array3;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Grid structure implementation.
/// Quantisation of the domain.
#[derive(Debug)]
pub struct Grid {
    /// Boundary.
    dom: Aabb,
    /// Cells.
    cells: Array3<Cell>,
}

impl Grid {
    #[pre(num_threads > 0)]
    #[pre(num_cells[X as usize] > 0)]
    #[pre(num_cells[Y as usize] > 0)]
    #[pre(num_cells[Z as usize] > 0)]
    pub fn new(num_threads: usize, num_cells: [usize; 3], dom: Aabb) -> Self {
        let total_cells = num_cells[X as usize] * num_cells[Y as usize] * num_cells[Z as usize];

        let bar = Arc::new(Mutex::new(ParallelBar::new(
            "Building cells",
            total_cells as u64,
            num_threads,
        )));
        let thread_ids: Vec<usize> = (0..num_threads).collect();
        let mut cell_lists: Vec<Vec<(usize, Cell)>> = thread_ids
            .par_iter()
            .map(|id| Self::init_cells(*id, Arc::clone(&bar), &num_cells, &dom))
            .collect();
        bar.lock().unwrap().finish_with_message("Cells built.");

        let mut bar = SerialBar::new("Sorting cells", total_cells as u64);
        let mut cells = Vec::with_capacity(total_cells);
        'outer: for n in 0..total_cells {
            bar.inc();

            for list in cell_lists.iter_mut() {
                if !list.is_empty() && list.last().unwrap().0 == n {
                    cells.push(list.pop().unwrap().1);
                    continue 'outer;
                }
            }
            panic!("Cell index {} missing.", n);
        }
        bar.finish_with_message("Cells sorted.");

        Self {
            dom,
            cells: Array3::from_shape_vec(num_cells, cells).unwrap(),
        }
    }

    /// Initialise the cells.
    #[pre(num_cells[X as usize] > 0)]
    #[pre(num_cells[Y as usize] > 0)]
    #[pre(num_cells[Z as usize] > 0)]
    fn init_cells(
        thread_id: usize,
        bar: Arc<Mutex<ParallelBar>>,
        num_cells: &[usize; 3],
        dom: &Aabb,
    ) -> Vec<(usize, Cell)> {
        let total_cells = num_cells[X as usize] * num_cells[Y as usize] * num_cells[Z as usize];

        let mut cell_size = dom.widths();
        for (w, n) in cell_size.iter_mut().zip(num_cells) {
            *w /= *n as f64;
        }

        let mut cells = Vec::new();
        while let Some(n) = bar.lock().unwrap().inc(thread_id) {
            let n = total_cells - 1 - n as usize;

            let zi = n % num_cells[X as usize];
            let yi = ((n - zi) / num_cells[X as usize]) % num_cells[Y as usize];
            let xi = (n - zi - (yi * num_cells[X as usize]))
                / (num_cells[X as usize] * num_cells[Y as usize]);

            let mins = dom.mins()
                + Vector3::new(
                    cell_size.x * xi as f64,
                    cell_size.y * yi as f64,
                    cell_size.z * zi as f64,
                );
            let maxs = mins + cell_size;

            cells.push((n, Cell::new(Aabb::new(mins, maxs))));
        }

        cells
    }
}
