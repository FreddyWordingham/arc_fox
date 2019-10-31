//! World domain structure.

use crate::geom::Aabb;

/// World domain structure.
/// All simulation is contained within the boundary of the domain.
#[derive(Debug)]
pub struct Domain {
    /// Number of splits along each axis.
    num_cells: [usize; 3],
    /// Boundary.
    boundary: Aabb,
}

impl Domain {
    /// Construct a new instance.
    pub fn new(num_cells: [usize; 3], boundary: Aabb) -> Self {
        Self {
            num_cells,
            boundary,
        }
    }
}
