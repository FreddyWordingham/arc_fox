//! Lightmap structure.

use super::Record;
use crate::base::Resolution;
use ndarray::Array3;
use std::ops::AddAssign;

/// Lightmap structure implementation.
/// Record Lightmap.
#[derive(Debug)]
pub struct Lightmap {
    /// Record array.
    pub recs: Array3<Record>,
}

impl Lightmap {
    /// Construct a new instance.
    pub fn new(res: Resolution) -> Self {
        Self {
            recs: Array3::from_elem(*res.arr(), Record::new()),
        }
    }
}

impl AddAssign<&Lightmap> for Lightmap {
    fn add_assign(&mut self, rhs: &Self) {
        self.recs += &rhs.recs;
    }
}
