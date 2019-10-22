//! Material mapping structure.

use std::collections::HashMap;
use crate::phy::Material;
use contracts::pre;

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
        Self {
            mats
        }
    }
}
