//! Grid structure.

use super::Cell;
use crate::{base::Resolution, geom::shape::Aabb, json, util::progress::bar};
use contracts::pre;
use log::info;
use nalgebra::{Point3, Vector3};
use ndarray::Array3;
use serde::{Deserialize, Serialize};

/// Grid structure implementation.
/// Quantisation of the domain.
#[derive(Debug)]
pub struct Grid {
    /// Boundary.
    aabb: Aabb,
    /// Resolution.
    res: Resolution,
    /// Cells.
    cells: Array3<Cell>,
}

impl Grid {
    /// Construct a new instance.
    pub fn new(aabb: Aabb, res: Resolution) -> Self {
        info!("Constructing the grid...");

        let mut cells = Vec::with_capacity(res.total());
        let bar = bar("Constructing cells", res.total() as u64);
        for _index in res.iter() {
            bar.inc(1);
            cells.push(Cell::new());
        }
        bar.finish_with_message(&format!("{} cells constructed.", res.total()));

        let cells = Array3::from_shape_vec(res.arr().clone(), cells)
            .expect("Unable to construct grid cells.");

        let grid = Self { aabb, res, cells };

        info!("Grid construction complete.\n");

        grid
    }

    /// Build a new instance.
    pub fn build(proto_grid: &ProtoGrid) -> Self {
        Self::new(
            Aabb::new_centred(&Point3::origin(), proto_grid.half_extents()),
            proto_grid.res().clone(),
        )
    }

    /// Reference the boundary.
    pub fn aabb(&self) -> &Aabb {
        &self.aabb
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
