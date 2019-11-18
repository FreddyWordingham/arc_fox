//! Grid structure.

use super::Cell;
use crate::{
    base::Resolution,
    geom::shape::Aabb,
    json,
    util::progress::bar,
    world::{InterMap, MolMap, RegionMap},
};
use contracts::pre;
use log::info;
use nalgebra::{Point3, Vector3};
use ndarray::Array3;
use serde::{Deserialize, Serialize};

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
    ) -> Self {
        info!("Constructing the grid...");

        let mut cell_size = dom.widths();
        for (w, n) in cell_size.iter_mut().zip(res.arr().iter()) {
            *w /= *n as f64;
        }

        let mut cells = Vec::with_capacity(res.total());
        let bar = bar("Constructing cells", res.total() as u64);
        for index in res.iter() {
            bar.inc(1);

            let mins = dom.mins()
                + Vector3::new(
                    cell_size.x * index.x() as f64,
                    cell_size.y * index.y() as f64,
                    cell_size.z * index.z() as f64,
                );
            let maxs = mins + cell_size;

            cells.push(Cell::new(
                &dom,
                inter_map,
                mol_map,
                region_map,
                Aabb::new(mins, maxs),
            ));
        }
        bar.finish_with_message(&format!("{} cells constructed.", res.total()));

        let cells = Array3::from_shape_vec(res.arr().clone(), cells)
            .expect("Unable to construct grid cells.");

        let grid = Self { dom, res, cells };

        info!("Grid construction complete.\n");

        grid
    }

    /// Build a new instance.
    #[pre(!inter_map.is_empty())]
    pub fn build(
        proto_grid: &ProtoGrid,
        inter_map: &'a InterMap,
        mol_map: &'a MolMap,
        region_map: &RegionMap,
    ) -> Self {
        Self::new(
            inter_map,
            mol_map,
            region_map,
            Aabb::new_centred(&Point3::origin(), proto_grid.half_extents()),
            proto_grid.res().clone(),
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
