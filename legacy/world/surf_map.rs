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
pub struct SurfMap<'a> {
    /// Surface hashmap.
    surfs: HashMap<String, Surface<'a>>,
}

impl<'a> SurfMap<'a> {
    /// Construct a new instance.
    #[pre(!surfs.is_empty())]
    pub fn new(surfs: HashMap<String, Surface<'a>>) -> Self {
        Self { surfs }
    }
}

impl<'a> Display for SurfMap<'a> {
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

impl<'a> Index<&str> for SurfMap<'a> {
    type Output = Surface<'a>;

    fn index(&self, key: &str) -> &Self::Output {
        &self.surfs[key]
    }
}
