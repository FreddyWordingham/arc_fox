//! World domain partitioning structure.

use crate::{geom::Cube, index::Layout3, phy::ThreeDimensional};
use contracts::pre;
use nalgebra::Vector3;
use std::fmt::{Display, Formatter, Result};

/// Domain with regular split grid partitioning.
#[derive(Debug)]
pub struct Domain {
    /// Boundary.
    boundary: Cube,
    /// Number of cells in each axis.
    layout: Layout3,
    /// Cell size.
    cell_size: Vector3<f64>,
}

impl Domain {
    /// Construct a new instance.
    #[pre(layout.x() > 0)]
    #[pre(layout.y() > 0)]
    #[pre(layout.z() > 0)]
    pub fn new(boundary: Cube, layout: Layout3) -> Self {
        let mut cell_size = boundary.widths();
        cell_size.x /= layout.x() as f64;
        cell_size.y /= layout.y() as f64;
        cell_size.z /= layout.z() as f64;

        Self {
            boundary,
            layout,
            cell_size,
        }
    }

    /// Reference the boundary geometry.
    pub fn boundary(&self) -> &Cube {
        &self.boundary
    }

    /// Reference the layout.
    pub fn layout(&self) -> &Layout3 {
        &self.layout
    }

    /// Reference the cell size.
    pub fn cell_size(&self) -> &Vector3<f64> {
        &self.cell_size
    }

    /// Construct the boundary of a cell at a given index.
    #[pre(index[0] < self.layout.x())]
    #[pre(index[1] < self.layout.y())]
    #[pre(index[2] < self.layout.z())]
    pub fn cell_boundary(&self, index: [usize; 3]) -> Cube {
        let mut min = self.boundary.mins().clone();
        min.x += self.cell_size.x * index[0] as f64;
        min.y += self.cell_size.y * index[1] as f64;
        min.z += self.cell_size.z * index[2] as f64;

        Cube::new(min, min + self.cell_size)
    }
}

impl Display for Domain {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mins = self.boundary.mins();
        let maxs = self.boundary.maxs();

        writeln!(f, "Mins     : {},\t{},\t{}", mins.x, mins.y, mins.z)?;
        writeln!(f, "Maxs     : {},\t{},\t{}", maxs.x, maxs.y, maxs.z)?;
        writeln!(
            f,
            "Cells    : {} - {} - {}",
            self.layout.x(),
            self.layout.y(),
            self.layout.z()
        )?;
        write!(
            f,
            "Num cells: {}",
            self.layout.x() * self.layout.y() * self.layout.z()
        )
    }
}
