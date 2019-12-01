//! Reactions-builder functions.

use crate::{sci::chem::ReactionBuilder, world::parts::load_map};
use contracts::pre;
use std::{collections::HashMap, path::Path};

/// Load a map of reaction-builders.
#[pre(dir.is_dir())]
pub fn load(dir: &Path, names: &[String]) -> HashMap<String, ReactionBuilder> {
    load_map(dir, names, "reaction")
}
