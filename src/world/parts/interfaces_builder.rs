//! Interfaces-builder functions.

use crate::world::{mat::InterfaceBuilder, parts::load_map};
use contracts::pre;
use std::{collections::HashMap, path::Path};

/// Load a map of interface-builders.
#[pre(dir.is_dir())]
pub fn load(dir: &Path, names: &[String]) -> HashMap<String, InterfaceBuilder> {
    load_map(dir, names, "interface")
}
