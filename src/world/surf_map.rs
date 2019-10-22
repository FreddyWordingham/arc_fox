//! Surface-mapping structure.

use crate::phy::Surface;
use contracts::pre;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
    ops::Index,
};

/// Structure storing all surfaces.
#[derive(Debug)]
pub struct SurfMap {
    /// Surface hashmap.
    surfs: HashMap<String, Surface>,
}

impl SurfMap {
    /// Construct a new instance.
    #[pre(!surfs.is_empty())]
    pub fn new(surfs: HashMap<String, Surface>) -> Self {
        Self { surfs }
    }
}

impl Display for SurfMap {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut iter = self.surfs.iter().peekable();
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

impl Index<&str> for SurfMap {
    type Output = Surface;

    fn index(&self, key: &str) -> &Self::Output {
        &self.surfs[key]
    }
}
