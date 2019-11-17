//! Archive structure.

use super::Record;
use crate::base::Resolution;
use ndarray::Array3;
use std::ops::AddAssign;

/// Archive structure implementation.
/// Record archive.
#[derive(Debug)]
pub struct Archive {
    /// Record array.
    recs: Array3<Record>,
}

impl Archive {
    /// Construct a new instance.
    pub fn new(res: Resolution) -> Self {
        Self {
            recs: Array3::from_elem(res.arr, Record::new()),
        }
    }
}

impl AddAssign<&Archive> for Archive {
    fn add_assign(&mut self, rhs: &Self) {
        self.recs += &rhs.recs;
    }
}
