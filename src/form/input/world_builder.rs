//! World builder input form.

use super::super::proto;
use crate::file::{as_json, from_json, Loadable, Saveable};
use serde::{Deserialize, Serialize};
use std::{env::var, path::Path};

/// Input form structure containing all information required to run the world_builder binary.
#[derive(Debug, Deserialize, Serialize)]
pub struct WorldBuilder {
    /// Directory information.
    dir: proto::Dir,
}

impl WorldBuilder {
    /// Create an example world.
    pub fn example() -> Self {
        Self {
            dir: proto::Dir::new(
                Some(format!(
                    "{}/cwd",
                    var("ARC_DIR").expect("Environment variable ARC_DIR is not set!")
                )),
                "out".to_string(),
                "res".to_string(),
                "mats".to_string(),
                "meshes".to_string(),
            ),
        }
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
