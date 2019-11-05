//! Archive structure.

use super::Record;
use crate::index::Layout;
use ndarray::Array3;

/// Archive datacube.
#[derive(Debug)]
pub struct Archive {
    /// Array of data records.
    recs: Array3<Record>,
}

impl Archive {
    /// Construct a new instance.
    pub fn new(layout: Layout) -> Self {
        Self {
            recs: Array3::from_elem(*layout.nis(), Record::new()),
        }
    }
}
