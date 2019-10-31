//! Domain cell structure.

use crate::{data::Record, geom::Aabb};

/// Domain cell structure.
/// Contains local spatial information.
#[derive(Debug)]
pub struct Cell {
    /// Boundary.
    boundary: Aabb,
    /// Data record.
    rec: Record,
}

impl Cell {
    /// Construct a new instance.
    pub fn new(boundary: Aabb) -> Self {
        Self {
            boundary,
            rec: Record::new(),
        }
    }

    /// Reference the data record.
    pub fn rec(&self) -> &Record {
        &self.rec
    }
}
