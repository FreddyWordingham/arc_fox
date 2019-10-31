//! Material map alias.

use super::Entity;
use std::collections::HashMap;

/// Material map alias type.
type MatMap<'a> = HashMap<&'static str, Entity<'a>>;
