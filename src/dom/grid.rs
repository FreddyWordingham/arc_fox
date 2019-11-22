//! Grid structure.

use super::Cell;
use crate::{
    base::Resolution,
    dom::State,
    geom::shape::Aabb,
    json,
    util::{progress::bar, Monitor},
    world::{InterMap, MolMap, RegionMap},
};
use contracts::pre;
use log::info;
use nalgebra::{Point3, Vector3};
use ndarray::Array3;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// Grid structure implementation.
/// Quantisation of the domain.
#[derive(Debug)]
pub struct Grid<'a> {
    /// Boundary.
    dom: Aabb,
    /// Resolution.
    res: Resolution,
    /// Cells.
    cells: Array3<Cell<'a>>,
}

impl<'a> Grid<'a> {
    /// Construct a new instance.
    #[pre(!inter_map.is_empty())]
    #[pre(!mol_map.is_empty())]
    #[pre(!region_map.is_empty())]
    pub fn new(
        inter_map: &'a InterMap,
        mol_map: &'a MolMap,
        region_map: &RegionMap,
        dom: Aabb,
        res: Resolution,
        num_threads: usize,
    ) -> Self {
        info!("Constructing grid...");

        let mut cell_size = dom.widths();
        for (w, n) in cell_size.iter_mut().zip(res.arr().iter()) {
            *w /= *n as f64;
        }

        let monitor = Arc::new(Mutex::new(Monitor::new(
            "Constructing cells",
            res.total() as u64,
            num_threads,
        )));

        let thread_ids: Vec<usize> = (0..num_threads).collect();
        let mut cell_lists: Vec<Vec<(usize, Cell<'a>)>> = thread_ids
            .par_iter()
            .map(|id| {
                Self::build_cells(
                    *id,
                    &res,
                    Arc::clone(&monitor),
                    &dom,
                    &cell_size,
                    inter_map,
                    mol_map,
                    region_map,
                )
            })
            .collect();
        monitor
            .lock()
            .unwrap()
            .finish_with_message("Cells constructed.");

        let mut cells = Vec::with_capacity(res.total());
        let pb = bar("Sorting cells", res.total() as u64);
        'outer: for n in 0..res.total() {
            pb.inc(1);

            for list in cell_lists.iter_mut() {
                if !list.is_empty() && list.last().unwrap().0 == n {
                    cells.push(list.pop().unwrap().1);
                    continue 'outer;
                }
            }

            panic!("Cell index {} is missing.", n);
        }
        pb.finish_with_message("Cells sorted.");

        let cells = Array3::from_shape_vec(res.arr().clone(), cells)
            .expect("Unable to construct grid cells.");

        let grid = Self { dom, res, cells };

        info!("Grid construction complete.\n");

        grid
    }

    /// Construct cells for the grid.
    fn build_cells(
        thread_id: usize,
        res: &Resolution,
        monitor: Arc<Mutex<Monitor>>,
        dom: &Aabb,
        cell_size: &Vector3<f64>,
        inter_map: &'a InterMap,
        mol_map: &'a MolMap,
        region_map: &RegionMap,
    ) -> Vec<(usize, Cell<'a>)> {
        let mut cells = Vec::new();

        while let Some(n) = monitor.lock().unwrap().inc(thread_id) {
            let n = res.total() - 1 - (n as usize);

            let index = res.nth_index(n as usize);

            let mins = dom.mins()
                + Vector3::new(
                    cell_size.x * index.x() as f64,
                    cell_size.y * index.y() as f64,
                    cell_size.z * index.z() as f64,
                );
            let maxs = mins + cell_size;

            cells.push((
                n,
                Cell::new(&dom, inter_map, mol_map, region_map, Aabb::new(mins, maxs)),
            ));
        }

        cells
    }

    /// Build a new instance.
    #[pre(!inter_map.is_empty())]
    #[pre(num_threads > 0)]
    pub fn build(
        proto_grid: &ProtoGrid,
        inter_map: &'a InterMap,
        mol_map: &'a MolMap,
        region_map: &RegionMap,
        num_threads: usize,
    ) -> Self {
        Self::new(
            inter_map,
            mol_map,
            region_map,
            Aabb::new_centred(&Point3::origin(), proto_grid.half_extents()),
            proto_grid.res().clone(),
            num_threads,
        )
    }

    /// Reference the domain boundary.
    pub fn dom(&self) -> &Aabb {
        &self.dom
    }

    /// Reference the grid resolution.
    pub fn res(&self) -> &Resolution {
        &self.res
    }

    /// Reference the cells.
    pub fn cells(&self) -> &Array3<Cell<'a>> {
        &self.cells
    }

    /// Create a state-cube of references.
    pub fn states(&self) -> Array3<&State> {
        let mut states = Vec::with_capacity(self.res.total());

        for cell in self.cells().iter() {
            states.push(cell.state());
        }

        Array3::from_shape_vec(*self.res.arr(), states)
            .expect("Could not form state reference cube.")
    }
}

/// Proto-Grid structure implementation.
/// Stores information required to build a grid.
#[derive(Debug, Deserialize, Serialize)]
pub struct ProtoGrid {
    /// Grid resolution.
    res: Resolution,
    /// Half-extents.
    half_extents: Vector3<f64>,
}

impl ProtoGrid {
    /// Construct a new instance.
    #[pre(half_extents.iter().all(|x| *x > 0.0))]
    pub fn new(res: Resolution, half_extents: Vector3<f64>) -> Self {
        Self { res, half_extents }
    }

    /// Reference the grid resolution.
    pub fn res(&self) -> &Resolution {
        &self.res
    }

    /// Reference the half-extents.
    pub fn half_extents(&self) -> &Vector3<f64> {
        &self.half_extents
    }
}

json!(ProtoGrid);
