//! Archive structure.

use super::Record;
use crate::index::Layout;
use contracts::pre;
use ndarray::Array3;
use std::ops::{Add, AddAssign};

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

impl Add<&Self> for Archive {
    type Output = Self;

    #[pre(self.recs.shape() == rhs.recs.shape())]
    fn add(self, rhs: &Self) -> Self {
        let mut recs = self.recs;
        recs += &rhs.recs;

        Self { recs }
    }
}

impl AddAssign<&Self> for Archive {
    fn add_assign(&mut self, rhs: &Self) {
        self.recs += &rhs.recs;
    }
}
