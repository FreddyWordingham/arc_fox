//! World builder input form.

use super::super::proto;
use crate::file::{as_json, from_json, Loadable, Saveable};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Input form structure containing all information required to run the world_builder binary.
#[derive(Debug, Deserialize, Serialize)]
pub struct WorldBuilder {
    /// Directory information.
    dir: proto::Dir,
}

impl WorldBuilder {
    /// Create an example world.
    pub fn example(dir: proto::Dir) -> Self {
        Self { dir }
    }

    /// Reference the directory proto-structure.
    pub fn dir(&self) -> &proto::Dir {
        &self.dir
    }
}

impl Saveable for WorldBuilder {
    fn save(&self, path: &Path) {
        as_json(self, path);
    }
}

impl Loadable for WorldBuilder {
    fn load(path: &Path) -> Self {
        from_json(path)
    }
}
