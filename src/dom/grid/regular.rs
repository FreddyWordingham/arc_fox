//! Regular implementation.

use crate::{
    access,
    dom::{Cell, Name, Set},
    geom::Aabb,
    uni::Interface,
};
use nalgebra::Point3;
use ndarray::Array3;
use std::fmt::{Display, Formatter, Result};

/// Grid sized partition scheme.
pub struct Regular {
    /// Boundary.
    bound: Aabb,
    /// Cells.
    cells: Array3<Cell>,
}

impl Regular {
    access!(bound, Aabb);
    access!(cells, Array3<Cell>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(bound: Aabb, shape: [usize; 3], _inters: &Set<Interface>) -> Self {
        let mut cell_size = bound.widths();
        for (w, n) in cell_size.iter_mut().zip(shape.iter()) {
            *w /= *n as f64;
        }

        let total_cells = shape[0] * shape[1] * shape[2];
        let mut cells = Vec::with_capacity(total_cells);
        for xi in 0..*shape.get(0).expect("Missing resolution.") {
            let x = cell_size
                .get(0)
                .expect("Missing resolution.")
                .mul_add(xi as f64, bound.mins().x);
            for yi in 0..*shape.get(1).expect("Missing resolution.") {
                let y = cell_size
                    .get(1)
                    .expect("Missing resolution.")
                    .mul_add(yi as f64, bound.mins().y);
                for zi in 0..*shape.get(2).expect("Missing resolution.") {
                    let z = cell_size
                        .get(2)
                        .expect("Missing resolution.")
                        .mul_add(zi as f64, bound.mins().z);

                    let mins = Point3::new(x, y, z);
                    let maxs = mins + cell_size;

                    let cell_bound = Aabb::new(mins, maxs);
                    // let cell_centre = cell_bound.centre();

                    cells.push(Cell::new(cell_bound, crate::dom::Name::new("???")));
                }
            }
        }

        Self {
            bound,
            cells: Array3::from_shape_vec(shape, cells)
                .expect("Failed to convert cell vector to an array3."),
        }
    }

    /// Create a map of material keys.
    #[inline]
    #[must_use]
    pub fn mat_names(&self) -> Array3<&Name> {
        self.cells.map(|c| c.mat())
    }
}

impl Display for Regular {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let shape = self.cells.shape();

        write!(
            fmt,
            "[{} x {} x {}] {}\tmins: ({}, {}, {})\tmaxs: ({}, {}, {})",
            shape[0],
            shape[1],
            shape[2],
            self.cells.len(),
            self.bound.mins().x,
            self.bound.mins().y,
            self.bound.mins().z,
            self.bound.maxs().x,
            self.bound.maxs().y,
            self.bound.maxs().z,
        )
    }
}
