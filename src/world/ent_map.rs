//! Entity map alias.

use super::Entity;
use std::collections::HashMap;

/// Entity map alias type.
pub type EntMap<'a> = HashMap<&'static str, Entity<'a>>;
