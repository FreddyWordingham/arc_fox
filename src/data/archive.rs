//! Archive structure.

use super::Record;
use crate::index::Layout;
use ndarray::Array3;
use std::ops::AddAssign;

/// Record archive.
pub struct Archive {
    /// Record array.
    pub recs: Array3<Record>,
}

impl Archive {
    /// Construct a new instance.
    pub fn new(layout: Layout) -> Self {
        Self {
            recs: Array3::from_elem(*layout.nis(), Record::new()),
        }
    }
}

impl AddAssign<&Archive> for Archive {
    fn add_assign(&mut self, rhs: &Self) {
        self.recs += &rhs.recs;
    }
}
