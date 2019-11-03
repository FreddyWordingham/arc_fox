//! Material map alias.

use crate::phys::Material;
use std::collections::HashMap;

/// Material map alias type.
pub type MatMap<'a> = HashMap<&'static str, Material>;
