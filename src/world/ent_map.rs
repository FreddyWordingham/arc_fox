//! Entity map alias.

use super::Entity;
use crate::{geom::Shape, phys::Material};
use contracts::pre;
use std::collections::HashMap;

/// Entity map alias type.
pub type EntMap<'a> = HashMap<String, Entity<'a>>;

#[pre(!names.is_empty())]
#[post(!ret.is_empty())]
pub fn load_ent_map<'a>(names: &Vec<(Shape, &'a Material, &'a Material)>) -> EntMap<'a> {
    let ent_map = EntMap::with_capacity(names.len());

    ent_map
}
