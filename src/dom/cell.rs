//! Cell structure.

use crate::{data::Record, geom::Aabb};
use contracts::pre;

/// Single domain cell.
pub struct Cell {
    /// Record.
    rec: Record,
    /// Boundary.
    aabb: Aabb,
}
