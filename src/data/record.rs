//! Data record structure.

use std::ops::{Add, AddAssign};

/// Data record.
#[derive(Debug, Clone)]
pub struct Record {}

impl Record {
    /// Construct a new instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Add for Record {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Self {}
    }
}

impl AddAssign for Record {
    fn add_assign(&mut self, _rhs: Self) {}
}
