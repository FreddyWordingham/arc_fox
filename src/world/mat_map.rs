//! Material-mapping structure.

use crate::phy::Material;
use contracts::pre;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
    ops::Index,
};

/// Structure storing all materials.
#[derive(Debug)]
pub struct MatMap {
    /// Material hashmap.
    mats: HashMap<String, Material>,
}

impl MatMap {
    /// Construct a new instance.
    #[pre(!mats.is_empty())]
    pub fn new(mats: HashMap<String, Material>) -> Self {
        Self { mats }
    }
}

impl Display for MatMap {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut iter = self.mats.iter().peekable();
        while let Some((key, _mat)) = iter.next() {
            if iter.peek().is_some() {
                writeln!(f, ">\t{}", key)?;
            } else {
                return write!(f, ">\t{}", key);
            }
        }

        unreachable!();
    }
}

impl Index<&str> for MatMap {
    type Output = Material;

    fn index(&self, key: &str) -> &Self::Output {
        &self.mats[key]
    }
}
